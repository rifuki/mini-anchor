use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Type};

pub fn derive_accounts_impl(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as DeriveInput);

    // Extract struct name and generics
    let struct_name = &input.ident;
    let generics = &input.generics;

    // Extract lifetime parameter
    let lifetime = generics
        .lifetimes()
        .next()
        .map(|lt| &lt.lifetime)
        .expect("Accounts struct must have a lifetime parameter");

    // Extract fields from the struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Accounts can only be derived for structs with named fields"),
        },
        _ => panic!("Accounts can only be derived for structs"),
    };

    // Generate code for each field
    let field_parsers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let is_mut = has_mut_attribute(field);

        generate_field_parser(field_name, field_type, is_mut)
    });

    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());

    quote! {
        impl #generics #struct_name #generics {
            pub fn try_accounts(
                accounts: &#lifetime [::mini_anchor::solana_program::account_info::AccountInfo<#lifetime>],
            ) -> Result<Self, ::mini_anchor::solana_program::program_error::ProgramError> {
                let mut index = 0;

                #(#field_parsers)*

                Ok(Self {
                    #(#field_names), *
                })
            }
        }
    }
    .into()
}

fn has_mut_attribute(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("account") {
            let mut is_mut = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("mut") {
                    is_mut = true;
                }
                Ok(())
            });
            return is_mut;
        }
    }
    false
}

fn generate_field_parser(
    field_name: &Ident,
    field_type: &Type,
    is_mut: bool,
) -> proc_macro2::TokenStream {
    let type_str = quote! { #field_type }.to_string();

    if type_str.contains("Signer") {
        quote! {
            let #field_name = {
                let info = accounts
                    .get(index)
                    .ok_or(::mini_anchor::solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
                index += 1;
                ::mini_anchor::Signer::new(info)
                    .map_err(|_| ::mini_anchor::solana_program::program_error::ProgramError::MissingRequiredSignature)?

            };
        }
    } else if type_str.contains("Account") {
        if is_mut {
            quote! {
                let #field_name = {
                    let info = accounts.get(index)
                        .ok_or(::mini_anchor::solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
                    if !info.is_writable {
                        return Err(::mini_anchor::solana_program::program_error::ProgramError::InvalidAccountData);
                    }
                    index += 1;
                    ::mini_anchor::Account::new(info)
                };
            }
        } else {
            quote! {
                let #field_name = {
                    let info = accounts.get(index)
                        .ok_or(::mini_anchor::solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
                    index += 1;
                    ::mini_anchor::Account::new(info)
                };
            }
        }
    } else if type_str.contains("Program") {
        quote! {
            let #field_name = {
                let info = accounts.get(index)
                    .ok_or(::mini_anchor::solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
                index += 1;
                ::mini_anchor::Program::new(info)
            };
        }
    } else {
        // Default case: treat as a generic account
        quote! {
            let #field_name = {
                let info = accounts.get(index)
                    .ok_or(::mini_anchor::solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
                index += 1;
                info.clone()
            };
        }
    }
}
