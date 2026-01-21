use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error as SynError};

pub fn declare_id_impl(input: TokenStream) -> TokenStream {
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
