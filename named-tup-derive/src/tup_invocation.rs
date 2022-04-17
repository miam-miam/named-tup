use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream, Result},
    Token,
};

use crate::tup_element::{TupDefault, TupElement, TupType};
use crate::IDENTIFIERS;

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
    pub fn into_token_stream(self) -> TokenStream {
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
                            .unwrap_or_else(|| syn::Expr::Verbatim(elem.1.name.to_token_stream())),
                    );
                    identifiers.push(elem.1.name);
                    generics
                        .push(syn::parse_str::<syn::Type>("named_tup::__private::Used").unwrap())
                }
                _ => {
                    expressions.push(syn::parse_str::<syn::Expr>("()").unwrap());
                    identifiers.push(Ident::new(identifier, Span::call_site()));
                    generics
                        .push(syn::parse_str::<syn::Type>("named_tup::__private::Unused").unwrap())
                }
            }
        }

        let expanded = quote! {
            named_tup::__private::Tup::<#(#empty),* , #(#generics),*>::new( #(#expressions),* )
        };
        expanded
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
                    match elem.1.default {
                        TupDefault::None => phantom_generics.push(
                            syn::parse_str::<syn::Type>("named_tup::__private::Used").unwrap(),
                        ),
                        TupDefault::Unfinished(expr) => {
                            return quote_spanned! {expr.span() => compile_error("Use the #[tup_default] attribute to automatically derive a TupDefault struct for each expression.");}
                        }
                        TupDefault::Finished(ident) => phantom_generics
                            .push(syn::parse2::<syn::Type>(ident.to_token_stream()).unwrap()),
                    }
                }
                _ => {
                    types.push(syn::parse_str::<syn::Type>("()").unwrap());
                    phantom_generics
                        .push(syn::parse_str::<syn::Type>("named_tup::__private::Unused").unwrap())
                }
            }
        }

        let expanded = quote! {
            named_tup::__private::Tup::<#(#types),* , #(#phantom_generics),*>
        };

        expanded
    }
}
