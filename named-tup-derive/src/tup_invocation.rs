use std::cmp::Ordering;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::{
    parse::{Parse, ParseStream, Result},
    Expr, Token, Type,
};

use crate::IDENTIFIERS;

#[derive(Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
pub struct TupElement {
    #[educe(Ord(method = "cmp"))]
    pub name: Ident,
    #[educe(Ord(ignore), PartialOrd(ignore), Eq(ignore), PartialEq(ignore))]
    pub value: Option<Expr>,
}

#[derive(Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
pub struct TupType {
    #[educe(Ord(method = "cmp"))]
    pub name: Ident,
    #[educe(Ord(ignore), PartialOrd(ignore), Eq(ignore), PartialEq(ignore))]
    pub value: Type,
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
        let value = match input.peek(Token![:]) {
            true => {
                input.parse::<Token![:]>()?;
                Some(input.parse()?)
            }
            false => None,
        };
        Ok(TupElement { name, value })
    }
}

impl Parse for TupType {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;
        Ok(TupType { name, value })
    }
}

pub enum TupInvocation {
    TupElement(Vec<TupElement>),
    TupType(Vec<TupType>),
}

impl Parse for TupInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        match fork.parse_terminated::<_, Token![,]>(TupType::parse) {
            Ok(v) => {
                input.advance_to(&fork);
                let mut values: Vec<TupType> = v.into_iter().collect();
                values.sort();
                Ok(TupInvocation::TupType(values))
            }
            Err(first_e) => match input.parse_terminated::<_, Token![,]>(TupElement::parse) {
                Ok(v) => {
                    let mut values: Vec<TupElement> = v.into_iter().collect();
                    values.sort();
                    Ok(TupInvocation::TupElement(values))
                }
                Err(mut second_e) => {
                    second_e.extend(first_e.into_iter());
                    Err(second_e)
                }
            },
        }
    }
}

impl TupInvocation {
    pub fn to_token_stream(self) -> TokenStream {
        match self {
            TupInvocation::TupElement(e) => Self::produce_expr(e),
            TupInvocation::TupType(t) => Self::produce_type(t),
        }
    }

    fn produce_expr(elements: Vec<TupElement>) -> TokenStream {
        let mut expressions = vec![];
        let mut identifiers = vec![];
        let mut generics = vec![];
        let empty = (0..IDENTIFIERS.len()).map(|_| Ident::new("_", Span::call_site()));
        let mut values = elements
            .into_iter()
            .map(|v| (v.name.to_string(), v))
            .peekable();

        for identifier in IDENTIFIERS {
            match values.peek() {
                Some((val, _)) if val == identifier => {
                    let elem = values.next().unwrap();
                    expressions.push(
                        elem.1
                            .value
                            .unwrap_or(syn::Expr::Verbatim(elem.1.name.to_token_stream())),
                    );
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

    fn produce_type(elements: Vec<TupType>) -> TokenStream {
        let mut types = vec![];
        let mut phantom_generics = vec![];
        let mut values = elements
            .into_iter()
            .map(|v| (v.name.to_string(), v))
            .peekable();

        for identifier in IDENTIFIERS {
            match values.peek() {
                Some((val, _)) if val == identifier => {
                    let elem = values.next().unwrap();
                    types.push(elem.1.value);
                    phantom_generics
                        .push(syn::parse_str::<syn::Type>("crate::named_tup::NotUnit").unwrap())
                }
                _ => {
                    types.push(syn::parse_str::<syn::Type>("()").unwrap());
                    phantom_generics.push(syn::parse_str::<syn::Type>("()").unwrap())
                }
            }
        }

        let expanded = quote! {
            crate::named_tup::Tup::<#(#types),* , #(#phantom_generics),*>
        };

        TokenStream::from(expanded)
    }
}
