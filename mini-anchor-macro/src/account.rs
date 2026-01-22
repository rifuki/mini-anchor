use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error as SynError, Fields, Meta};

pub fn account_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);

    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();
    let discriminator = generate_discriminator(&struct_name_str);

    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return SynError::new(input.span(), "Only named fields are supported")
                .to_compile_error()
                .into();
        }
    };

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut space_calculation = Vec::new();

    for field in fields.iter() {
        let field_name = match field.ident.as_ref() {
            Some(name) => name,
            None => {
                return SynError::new(field.span(), "Expected named field")
                    .to_compile_error()
                    .into();
            }
        };
        let field_type = &field.ty;

        field_names.push(field_name);
        field_types.push(field_type);

        // Check for #[max_len(N)] attribute
        let max_len = extract_max_length(&field.attrs);

        // Generate space calculation based on type and max_len
        let space_calc = if is_string_type(field_type) {
            match max_len {
                Some(len) => quote! { 4 + #len },
                None => {
                    return SynError::new(
                        field.span(),
                        "String fields must have a #[max_len(N)] attribute",
                    )
                    .into_compile_error()
                    .into();
                }
            }
        } else if is_vec_type(field_type) {
            // Extract inner type of Vec<T>
            let inner_type = extract_vec_inner_type(field_type);

            match (max_len, inner_type) {
                (Some(len), Some(inner)) => {
                    if is_vec_type(inner) {
                        return SynError::new(field.span(), "Nested Vec<Vec<T>> is not supported")
                            .into_compile_error()
                            .into();
                    };
                    quote! { 4 + (#len * std::mem::size_of::<#inner>()) }
                }
                (None, _) => {
                    return SynError::new(
                        field.span(),
                        "Vec fields require #[max_len(N)] attribute",
                    )
                    .into_compile_error()
                    .into()
                }
                (_, None) => {
                    return SynError::new(field.span(), "Unable to determine inner type of Vec")
                        .into_compile_error()
                        .into()
                }
            }
        } else if is_option_type(field_type) {
            let inner_type = match extract_option_inner_type(field_type) {
                Some(inner) => inner,
                None => {
                    return SynError::new(field.span(), "Unable to parse Option<T> inner type")
                        .into_compile_error()
                        .into()
                }
            };

            // Disallow nested Option<Option<T>>
            if is_option_type(inner_type) {
                return SynError::new(field.span(), "Nested Option<Option<T>> is not supported")
                    .into_compile_error()
                    .into();
            }

            if is_string_type(inner_type) {
                match max_len {
                    Some(len) => quote! { 1 + (4 + #len) },
                    None => {
                        return SynError::new(
                            field.span(),
                            "Option<String> fields must have a #[max_len(N)] attribute",
                        )
                        .into_compile_error()
                        .into()
                    }
                }
            } else if is_vec_type(inner_type) {
                let vec_inner = extract_vec_inner_type(inner_type);
                match (max_len, vec_inner) {
                    (Some(len), Some(vec_inner_type)) => {
                        quote! { 1 + (4 + (#len * std::mem::size_of::<#vec_inner_type>())) }
                    }
                    (None, _) => {
                        return SynError::new(
                            field.span(),
                            "Option<Vec<T>> fields require #[max_len(N)] attribute",
                        )
                        .into_compile_error()
                        .into()
                    }
                    (_, None) => {
                        return SynError::new(
                            field.span(),
                            "Unable to determine inner type of Vec inside Option",
                        )
                        .into_compile_error()
                        .into()
                    }
                }
            } else {
                quote! { 1 + std::mem::size_of::<#inner_type>() }
            }
        } else {
            quote! { std::mem::size_of::<#field_type>()}
        };

        space_calculation.push(space_calc);
    }

    let vis = &input.vis;

    let attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("max_len"))
        .collect();

    quote! {
        #(#attrs)*
        #[derive(Debug, Clone)]
        #vis struct #struct_name {
            #(pub #field_names: #field_types),*
        }

        impl #struct_name {
            // 8-byte discriminator
            pub const DISCRIMINATOR: [u8; 8] = [#(#discriminator),*];

            // Dynamic space calculation with max_len support!
            pub const SPACE: usize = 8 #(+ #space_calculation)*;

            // Wrapper for serialization
            pub fn try_serialize(&self, buf: &mut [u8]) -> Result<(), ::mini_anchor::solana_program::program_error::ProgramError> {
                <Self as ::mini_anchor::AnchorSerialize>::serialize(self, buf)?;
                Ok(())
            }

            // Wrapper for deserialization
            pub fn try_deserialize(data: &[u8]) -> Result<Self, ::mini_anchor::solana_program::program_error::ProgramError> {
                let (instance, _size) = <Self as ::mini_anchor::AnchorDeserialize>::deserialize(data)?;
                Ok(instance)
            }
        }

        // Serialize implementation
        impl ::mini_anchor::AnchorSerialize for #struct_name {
            // Serialize into a byte slice
            fn serialize(&self, buf: &mut [u8]) -> Result<usize, ::mini_anchor::solana_program::program_error::ProgramError> {
                if buf.len() < Self::SPACE {
                    return Err(::mini_anchor::solana_program::program_error::ProgramError::AccountDataTooSmall);
                }

                // Write discriminator
                buf[..8].copy_from_slice(&Self::DISCRIMINATOR);
                let mut offset = 8;

                #(
                    {
                        let written = <#field_types as ::mini_anchor::AnchorSerialize>::serialize(
                            &self.#field_names,
                            &mut buf[offset..]
                        );
                        offset += written?;
                    }
                )*

                Ok(offset)
            }
        }

        // Deserialize implementation
        impl ::mini_anchor::AnchorDeserialize for #struct_name {
            // Deserialize from a byte slice
            fn deserialize(data: &[u8]) -> Result<(Self, usize), ::mini_anchor::solana_program::program_error::ProgramError> {
                if data.len() < 8 { // At least need discriminator
                    return Err(::mini_anchor::solana_program::program_error::ProgramError::AccountDataTooSmall);
                }

                // Check discriminator
                if data[..8] != Self::DISCRIMINATOR {
                    return Err(::mini_anchor::solana_program::program_error::ProgramError::InvalidAccountData);
                }

                let mut offset = 8;

                Ok((Self {
                    #(
                        #field_names: {
                            let (value, read) = <#field_types as ::mini_anchor::AnchorDeserialize>::deserialize(&data[offset..])?;
                            offset += read;
                            value
                        }

                    ),*
                }, offset))
            }
        }
    }
    .into()
}

fn generate_discriminator(name: &str) -> [u8; 8] {
    use sha2::{Digest, Sha256};

    let preimage = format!("account:{name}");
    let hash = Sha256::digest(preimage.as_bytes());

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash[..8]);
    discriminator
}

fn extract_max_length(attrs: &[syn::Attribute]) -> Option<usize> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("max_len") {
            return None;
        }

        let Meta::List(meta) = &attr.meta else {
            return None;
        };
        let expr = syn::parse2::<syn::Expr>(meta.tokens.clone()).ok()?;
        let syn::Expr::Lit(expr_lit) = expr else {
            return None;
        };
        let syn::Lit::Int(lit) = expr_lit.lit else {
            return None;
        };

        lit.base10_parse::<usize>().ok()
    })
}

fn extract_vec_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;
    if segment.ident != "Vec" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    match args.args.iter().collect::<Vec<_>>().as_slice() {
        [syn::GenericArgument::Type(inner_type)] => Some(inner_type),
        _ => None,
    }
}

fn extract_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    match args.args.iter().collect::<Vec<_>>().as_slice() {
        [syn::GenericArgument::Type(inner_type)] => Some(inner_type),
        _ => None,
    }
}

fn is_string_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };
    segment.ident == "String"
}

fn is_vec_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };
    segment.ident == "Vec"
}

fn is_option_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };
    segment.ident == "Option"
}
