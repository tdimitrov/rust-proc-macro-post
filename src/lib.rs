use proc_macro::TokenStream;
use syn::parse_macro_input;

mod my_proc;

#[proc_macro]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    my_proc::my_proc_impl(input).into()
}
