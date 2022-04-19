use proc_macro2::{Ident, Span, TokenStream};

use crate::IDENTIFIERS;

pub fn to_token_stream() -> TokenStream {
    let generics: Vec<Ident> = IDENTIFIERS
        .iter()
        .enumerate()
        .map(|(count, _)| Ident::new(&*format!("T{count}"), Span::call_site()))
        .chain(
            IDENTIFIERS
                .iter()
                .enumerate()
                .map(|(count, _)| Ident::new(&*format!("P{count}"), Span::call_site())),
        )
        .collect();

    quote! {
        mod private {
            pub trait Sealed{}
            impl<#(#generics),*> Sealed for crate::tup_struct::Tup<#(#generics),*> {}
        }
    }
}
