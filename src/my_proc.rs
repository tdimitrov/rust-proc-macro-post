use proc_macro2::TokenStream;
use quote::quote;

pub fn my_proc_impl(input: TokenStream) -> TokenStream {
    quote!(println!("Answer: {}", #input))
}
