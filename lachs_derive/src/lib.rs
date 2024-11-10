#[doc = include_str!("../README.md")]
mod token;

use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token};
use token::*;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn token(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(_attr with Punctuated::<Ident, Token![,]>::parse_terminated);
    let ast = syn::parse(input).expect("#[token] currently only works for items!");

    impl_token_macro(ast, attrs)
}
