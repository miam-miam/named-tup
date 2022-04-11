use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;

use crate::IDENTIFIERS;

pub(crate) struct TupInfo {
    pub fields: Vec<Ident>,
    pub generics: Vec<Ident>,
    pub phantom_generics: Vec<Ident>,
}

impl TupInfo {
    pub(crate) fn new() -> TupInfo {
        let fields = IDENTIFIERS
            .iter()
            .map(|i| Ident::new(i, Span::call_site()))
            .collect();
        let generics = IDENTIFIERS
            .iter()
            .enumerate()
            .map(|(count, field)| Ident::new(&*format!("T{count}"), field.span()))
            .collect();
        let phantom_generics = IDENTIFIERS
            .iter()
            .enumerate()
            .map(|(count, field)| Ident::new(&*format!("P{count}"), field.span()))
            .collect();

        TupInfo {
            fields,
            generics,
            phantom_generics,
        }
    }

    fn to_def(&self) -> TokenStream {
        let (generics, fields, phantom_generics) =
            (&self.generics, &self.fields, &self.phantom_generics);
        let full_generics = generics.iter().chain(phantom_generics.iter());

        let expanded = quote! {
            pub struct Tup<#(#full_generics),*> {
                #(pub #fields: #generics),*,
                _phantom: core::marker::PhantomData<(#(#phantom_generics),*)>
            }
        };

        TokenStream::from(expanded)
    }

    fn to_new_impl(&self) -> TokenStream {
        let (generics, fields, phantom_generics) =
            (&self.generics, &self.fields, &self.phantom_generics);
        let full_generics0 = generics.iter().chain(phantom_generics.iter());
        let full_generics1 = generics.iter().chain(phantom_generics.iter());

        let expanded = quote! {
            impl<#(#full_generics0),*> Tup<#(#full_generics1),*> {
                pub fn new(#(#fields: #generics),*) -> Self{
                    Tup {
                        #(#fields),*,
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        TokenStream::from(expanded)
    }

    pub fn to_token_stream(&self) -> TokenStream {
        let mut result = self.to_def();
        result.extend(self.to_new_impl());
        result
    }
}
