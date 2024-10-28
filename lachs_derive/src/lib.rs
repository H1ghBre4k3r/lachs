#[doc = include_str!("../README.md")]
mod token;

use token::*;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn token(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("#[token] currently only works for items!");

    impl_token_macro(ast)
}
