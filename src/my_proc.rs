use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

pub fn my_proc_impl(input: TokenStream) -> TokenStream {
    quote_spanned!(input.span() => compile_error!("I don't like this...");)
}
