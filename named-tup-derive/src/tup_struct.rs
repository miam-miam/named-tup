use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;

use crate::IDENTIFIERS;

fn produce_tup_struct() -> TokenStream {
    let fields = IDENTIFIERS.iter().map(|i| Ident::new(i, Span::call_site()));
    let generics_vec: Vec<Ident> = IDENTIFIERS
        .iter()
        .enumerate()
        .map(|(count, field)| Ident::new(&*format!("T{count}"), field.span()))
        .collect();
    let generics = &generics_vec;

    let expanded = quote! {
        pub struct Tup<#(#generics),*> {
            #(pub #fields: #generics),*
        }
    };

    TokenStream::from(expanded)
}

pub fn to_token_stream() -> TokenStream {
    produce_tup_struct()
}
