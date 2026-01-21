mod account;
mod accounts;
mod declare_id;

use proc_macro::TokenStream;

#[proc_macro]
pub fn declare_id(input: TokenStream) -> TokenStream {
    declare_id::declare_id_impl(input)
}

#[proc_macro_attribute]
pub fn account(_attr: TokenStream, item: TokenStream) -> TokenStream {
    account::account_impl(item)
}

#[proc_macro_derive(Accounts, attributes(account))]
pub fn derive_accounts(input: TokenStream) -> TokenStream {
    accounts::derive_accounts_impl(input)
}
