extern crate core;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use std::env;

use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, Item};

use crate::tup_default::TupDefaultReplace;
use crate::tup_invocation::{TupElementInvocation, TupTypeInvocation};

mod sealed;
mod tup_default;
mod tup_element;
mod tup_invocation;
mod tup_struct;

const IDENTIFIERS: &[&str] = include!(concat!(env!("OUT_DIR"), "/identifiers.in"));

#[proc_macro]
pub fn tup_struct_builder(_input: TokenStream) -> TokenStream {
    TokenStream::from(tup_struct::TupInfo::new().to_token_stream())
}

#[proc_macro]
pub fn sealed_trait_builder(_input: TokenStream) -> TokenStream {
    TokenStream::from(sealed::to_token_stream())
}

#[proc_macro]
pub fn tup(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        return quote! {named_tup::__private::Tup::default()}.into();
    }
    let input = parse_macro_input!(input as TupElementInvocation);
    TokenStream::from(input.into_token_stream())
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Tup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TupTypeInvocation);
    TokenStream::from(input.into_token_stream())
}

#[proc_macro_attribute]
pub fn tup_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut syntax_tree = parse_macro_input!(item as Item);
    let mut replace = TupDefaultReplace::default();
    replace.visit_item_mut(&mut syntax_tree);
    let struct_invocations = replace.struct_invocations;
    let expanded = quote! {
        #struct_invocations
        #syntax_tree
    };
    expanded.into()
}
