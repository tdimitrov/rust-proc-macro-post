use proc_macro2::TokenStream;
use quote::quote;

pub fn my_proc_impl(input: TokenStream) -> TokenStream {
    let mut result = Vec::new();

    result.push(progress_message("Thinking about the answer".to_string()));
    result.push(answer(input));

    quote!(
        #(#result);*
    )
}

fn progress_message(msg: String) -> TokenStream {
    quote!(println!(#msg))
}

fn answer(result: TokenStream) -> TokenStream {
    quote!(println!("Answer: {}", #result))
}
