use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ExprMacro};

pub fn my_proc_impl(input: TokenStream) -> TokenStream {
    let progress = progress_message("Thinking about the answer".to_string());
    let answer = answer(input);

    quote!(
        #progress;
        #answer;
    )
}

fn progress_message(msg: String) -> ExprMacro {
    parse_quote!(println!(#msg))
}

fn answer(result: TokenStream) -> ExprMacro {
    parse_quote!(println!("Answer: {}", #result))
}
