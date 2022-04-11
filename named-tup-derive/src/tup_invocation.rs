use std::cmp::Ordering;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{
    parse::{Parse, ParseStream, Result},
    Expr, Token,
};

use crate::IDENTIFIERS;

#[derive(Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
pub struct TupElement {
    #[educe(Ord(method = "cmp"))]
    pub name: Ident,
    #[educe(Ord(ignore), PartialOrd(ignore), Eq(ignore), PartialEq(ignore))]
    pub value: Expr,
}

fn cmp(a: &Ident, b: &Ident) -> Ordering {
    let a = a.to_string();
    let b = b.to_string();
    if a > b {
        Ordering::Less
    } else if a < b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl Parse for TupElement {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;
        Ok(TupElement { name, value })
    }
}

pub struct TupInvocation {
    pub values: Vec<TupElement>,
}

impl Parse for TupInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse_terminated(TupElement::parse).map(
            |v: syn::punctuated::Punctuated<TupElement, Token![,]>| {
                let mut values: Vec<TupElement> = v.into_iter().collect();
                values.sort();
                TupInvocation { values }
            },
        )
    }
}

impl TupInvocation {
    pub fn to_token_stream(self) -> TokenStream {
        let mut expressions = vec![];
        let mut identifiers = vec![];
        let mut generics = vec![];
        let empty = (0..IDENTIFIERS.len()).map(|_| Ident::new("_", Span::call_site()));
        let mut values = self
            .values
            .into_iter()
            .map(|v| (v.name.to_string(), v))
            .peekable();

        for identifier in IDENTIFIERS {
            match values.peek() {
                Some((val, _)) if val == identifier => {
                    let elem = values.next().unwrap();
                    expressions.push(elem.1.value);
                    identifiers.push(elem.1.name);
                    generics.push(syn::parse_str::<syn::Type>("crate::named_tup::NotUnit").unwrap())
                }
                _ => {
                    expressions.push(syn::parse_str::<syn::Expr>("()").unwrap());
                    identifiers.push(Ident::new(identifier, Span::call_site()));
                    generics.push(syn::parse_str::<syn::Type>("()").unwrap())
                }
            }
        }

        let expanded = quote! {
            crate::named_tup::Tup::<#(#empty),* , #(#generics),*>::new( #(#expressions),* )
        };

        TokenStream::from(expanded)
    }
}
