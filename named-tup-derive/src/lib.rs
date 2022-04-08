extern crate core;
#[macro_use]
extern crate educe;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use std::env;

use syn::parse_macro_input;

use crate::tup_invocation::TupInvocation;

mod tup_invocation;
mod tup_struct;

const IDENTIFIERS: &'static [&'static str] = &include!(concat!(env!("OUT_DIR"), "/identifiers.in"));

#[proc_macro]
pub fn tup_struct_builder(_input: TokenStream) -> TokenStream {
    TokenStream::from(tup_struct::to_token_stream())
}

#[proc_macro]
pub fn tup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TupInvocation);
    TokenStream::from(input.to_token_stream())
}
