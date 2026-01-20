use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error as SynError, Fields, Meta};

#[proc_macro]
pub fn declare_id(input: TokenStream) -> TokenStream {
    let id_litstr = parse_macro_input!(input as syn::LitStr);
    let id_value = id_litstr.value();

    let decoded_id = match bs58::decode(id_value).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return SynError::new(id_litstr.span(), "Invalid Base58 string")
                .to_compile_error()
                .into();
        }
    };

    if decoded_id.len() != 32 {
        return SynError::new(id_litstr.span(), "Program ID must be 32 bytes long")
            .to_compile_error()
            .into();
    };

    let decoded_array: [u8; 32] = match decoded_id.try_into() {
        Ok(array) => array,
        Err(_) => {
            return SynError::new(id_litstr.span(), "Failed to convert to 32-byte array")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        pub const ID_BYTES: [u8; 32] = [#(#decoded_array),*];
        pub static ID: ::mini_anchor::solana_program::pubkey::Pubkey =
            ::mini_anchor::solana_program::pubkey::Pubkey::new_from_array(ID_BYTES);

        pub fn check_id(pubkey: &::mini_anchor::solana_program::pubkey::Pubkey) -> bool {
            pubkey == &ID
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn account(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
        let type_str = quote!(#field_type).to_string();

        let space_calc = if type_str.contains("String") {
            match max_len {
                Some(len) => quote! { 4 + #len },
                None => {
                    return SynError::new(
                        field.span(),
                        "String fields must have a #[max_len(N)] attribute"
                    ).into_compile_error()
                        .into();
                }
            }
        } else if type_str.contains("Vec") {
            match max_len {
                Some(len) => quote! { (4 + #len) },
                None => { return SynError::new(
                    field.span(),
                    "Vec fields require #[max_len(N)] attribute"
                ).into_compile_error().into()}
            }
        } else {
            quote! { std::mem::size_of::<#field_type>()}
        };

        space_calculation.push(space_calc);
    }

    let vis = &input.vis;

    let attrs: Vec<_> = input.attrs.iter().filter(|a| !a.path().is_ident("max_len")).collect();

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

            // Serialize to bytes (safe, no unsafe)
            pub fn try_serialize(&self, buf: &mut [u8]) -> Result<(), &'static str> {
                if buf.len() < Self::SPACE {
                    return Err("Buffer too small for account");
                }

                // Write discriminator
                buf[..8].copy_from_slice(&Self::DISCRIMINATOR);
                let mut offset = 8;

                // Write each field using trait
                #(
                    {
                        let written = <#field_types as ::mini_anchor::AnchorSerialize>::serialize(
                            &self.#field_names,
                            &mut buf[offset..] // This is safe as we checked buffer size above

                        )?;
                        offset += written;
                    }
                )*

                Ok(())
            }

            pub fn try_deserialize(data: &[u8]) -> Result<Self, &'static str> {
                if data.len() < Self::SPACE {
                    return Err("Data too short for account");
                }

                // Check discriminator
                if data[..8] != Self::DISCRIMINATOR {
                    return Err("Discriminator mismatch");
                }

                let mut offset = 8;

                // Read each field using trait
                Ok(Self {
                    #(
                        #field_names: {
                            let (value, read) = <#field_types as ::mini_anchor::AnchorDeserialize>::deserialize(&data[offset..])?;
                            offset += read; 
                            value
                        }
                    ),*
                })
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
    for attr in attrs {
        if attr.path().is_ident("max_len") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.clone();
                if let Ok(expr) = syn::parse2::<syn::Expr>(tokens) {
                    if let syn::Expr::Lit(expr_lit) = expr {
                        if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                            return lit_int.base10_parse::<usize>().ok();
                        }
                    }
                }
            }
        } 
    }
    None
}
