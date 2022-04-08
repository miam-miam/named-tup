use std::cmp::Ordering;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{
    Expr,
    parse::{Parse, ParseStream, Result}, Token,
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
                let mut values: Vec<_> = v.into_iter().collect();
                values.sort();
                TupInvocation { values }
            },
        )
    }
}

impl TupInvocation {
    pub fn to_token_stream(mut self) -> TokenStream {
        let mut expressions = vec![];
        let mut identifiers = vec![];

        // We need to go backwards so that values and identifiers are sorted in the same way and so that popping is O(1).
        for identifier in IDENTIFIERS.iter().rev() {
            match self
                .values
                .get(self.values.len().saturating_sub(1))
                .map(|v| v.name.to_string())
            {
                Some(tup_identifier) if identifier == &tup_identifier => {
                    let elem = self.values.pop().unwrap();
                    expressions.push(elem.value);
                    identifiers.push(elem.name);
                }
                _ => {
                    expressions.push(syn::parse_str::<syn::Expr>("()").unwrap());
                    identifiers.push(Ident::new(identifier, Span::call_site()));
                }
            };
        }
        let expanded = quote! {
            crate::named_tup::Tup {
                #(#identifiers: #expressions),*
            }
        };

        TokenStream::from(expanded)
    }
}
