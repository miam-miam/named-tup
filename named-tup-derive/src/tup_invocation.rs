use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_quote, Token, Type,
};

use crate::tup_element::{TupDefault, TupElement, TupType};
use crate::IDENTIFIERS;

pub struct TupElementInvocation(Vec<TupElement>);

pub struct TupTypeInvocation(Vec<TupType>);

impl Parse for TupElementInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values: Vec<TupElement> = input
            .parse_terminated::<_, Token![,]>(TupElement::parse)?
            .into_iter()
            .collect();
        values.sort();
        Ok(TupElementInvocation(values))
    }
}

impl Parse for TupTypeInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values: Vec<TupType> = input
            .parse_terminated::<_, Token![,]>(TupType::parse)?
            .into_iter()
            .collect();
        values.sort();
        Ok(TupTypeInvocation(values))
    }
}

impl TupElementInvocation {
    pub fn into_token_stream(self) -> TokenStream {
        let mut expressions = vec![];
        let mut identifiers = vec![];
        let mut generics: Vec<Type> = vec![];
        let empty = (0..IDENTIFIERS.len()).map(|_| Ident::new("_", Span::call_site()));
        let mut values = self
            .0
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
                    generics.push(parse_quote!(named_tup::__private::Used))
                }
                _ => {
                    expressions.push(parse_quote!(()));
                    identifiers.push(Ident::new(identifier, Span::call_site()));
                    generics.push(parse_quote!(named_tup::__private::Unused))
                }
            }
        }

        assert!(
            values.next().is_none(),
            "tup! invocation contained identifiers that did not match to any known identifiers"
        );

        let expanded = quote! {
            named_tup::__private::Tup::<#(#empty),* , #(#generics),*>::new( #(#expressions),* )
        };
        expanded
    }
}

impl TupTypeInvocation {
    pub fn into_token_stream(self) -> TokenStream {
        let mut types = vec![];
        let mut phantom_generics = vec![];
        let mut values = self
            .0
            .into_iter()
            .map(|v| (v.name.to_string(), v))
            .peekable();

        for identifier in IDENTIFIERS {
            match values.peek() {
                Some((val, _)) if val == identifier => {
                    let elem = values.next().unwrap();
                    types.push(elem.1.value);
                    match elem.1.default {
                        TupDefault::None => {
                            phantom_generics.push(parse_quote!(named_tup::__private::Used))
                        }
                        TupDefault::Unfinished(expr) => {
                            return quote_spanned! {expr.span() => compile_error("Use the #[tup_default] attribute to automatically derive a TupDefault struct for each expression.");};
                        }
                        TupDefault::Finished(ident) => phantom_generics
                            .push(syn::parse2::<Type>(ident.to_token_stream()).unwrap()),
                    }
                }
                _ => {
                    types.push(parse_quote!(()));
                    phantom_generics.push(parse_quote!(named_tup::__private::Unused));
                }
            }
        }

        assert!(
            values.next().is_none(),
            "Tup! invocation contained identifiers that did not match to any known identifiers"
        );

        let expanded = quote! {
            named_tup::__private::Tup::<#(#types),* , #(#phantom_generics),*>
        };

        expanded
    }
}
