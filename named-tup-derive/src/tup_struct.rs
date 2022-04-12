use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;

use crate::IDENTIFIERS;

pub(crate) struct TupInfo {
    pub fields: Vec<Ident>,
    pub generics: Vec<Ident>,
    pub phantom_generics: Vec<Ident>,
    pub full_generics: TokenStream,
}

impl TupInfo {
    pub(crate) fn new() -> TupInfo {
        let fields = IDENTIFIERS
            .iter()
            .map(|i| Ident::new(i, Span::call_site()))
            .collect();
        let generics: Vec<Ident> = IDENTIFIERS
            .iter()
            .enumerate()
            .map(|(count, field)| Ident::new(&*format!("T{count}"), field.span()))
            .collect();
        let phantom_generics: Vec<Ident> = IDENTIFIERS
            .iter()
            .enumerate()
            .map(|(count, field)| Ident::new(&*format!("P{count}"), field.span()))
            .collect();
        let full_generics = generics.iter().chain(phantom_generics.iter());

        let full_generics = quote! {
            #(#full_generics),*
        };

        TupInfo {
            fields,
            generics,
            phantom_generics,
            full_generics,
        }
    }

    fn to_def(&self) -> TokenStream {
        let (generics, fields, phantom_generics, full_generics) = (
            &self.generics,
            &self.fields,
            &self.phantom_generics,
            &self.full_generics,
        );

        // TODO: Do Debug properly
        let expanded = quote! {
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            #[must_use]
            pub struct Tup<#full_generics> {
                #(pub #fields: #generics),*,
                _phantom: core::marker::PhantomData<(#(#phantom_generics),*)>
            }
        };

        TokenStream::from(expanded)
    }

    fn to_new_impl(&self) -> TokenStream {
        let (generics, fields, full_generics) = (&self.generics, &self.fields, &self.full_generics);

        let expanded = quote! {
            impl<#full_generics> Tup<#full_generics> {
                pub fn new(#(#fields: #generics),*) -> Self {
                    Tup {
                        #(#fields),*,
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        TokenStream::from(expanded)
    }

    fn to_default_impl(&self) -> TokenStream {
        let fields = &self.fields;
        let generics = (0..self.generics.len()).map(|_| syn::parse_str::<syn::Type>("()").unwrap());
        let phantom_generics = (0..self.phantom_generics.len())
            .map(|_| syn::parse_str::<syn::Type>("crate::NotUnit").unwrap());

        let expanded = quote! {
            impl core::default::Default for Tup<#(#generics),*,#(#phantom_generics),*> {
                fn default() -> Self {
                    Tup {
                        #(#fields: ()),*,
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        TokenStream::from(expanded)
    }

    fn to_debug_impl(&self) -> TokenStream {
        let (generics, fields, full_generics, phantom_generics) = (
            &self.generics,
            &self.fields,
            &self.full_generics,
            &self.phantom_generics,
        );

        let expanded = quote! {
            impl<'a, #full_generics> core::fmt::Debug for Tup<#full_generics>
                where #((&'a #generics, #phantom_generics, &'static str): crate::ConvertToDebugStruct),*,
                #(#phantom_generics: core::default::Default),*,
                #(#generics: 'a),*,
            {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    let mut debug_struct = f.debug_struct("tup");
                    #((&self.#fields, <#phantom_generics as core::default::Default>::default(), stringify!(#fields)).convert(&mut debug_struct);)*
                    debug_struct.finish()
                }
            }
        };

        println!("{expanded}");

        TokenStream::from(expanded)
    }

    fn to_add_impl(&self) -> TokenStream {
        let (generics, fields, phantom_generics, full_generics) = (
            &self.generics,
            &self.fields,
            &self.phantom_generics,
            &self.full_generics,
        );

        let rhs_generics_stored: Vec<Ident> =
            generics.iter().map(|g| format_ident!("RHS{g}")).collect();
        let rhs_generics = &rhs_generics_stored;

        let rhs_phantom_generics_stored: Vec<Ident> = phantom_generics
            .iter()
            .map(|g| format_ident!("RHS{g}"))
            .collect();
        let rhs_phantom_generics = &rhs_phantom_generics_stored;

        let full_rhs_generics = generics
            .iter()
            .chain(phantom_generics.iter())
            .map(|g| format_ident!("RHS{g}"));
        let full_rhs_generics = &quote! {#(#full_rhs_generics),*};

        let expanded = quote! {
            impl<#full_generics, #full_rhs_generics> core::ops::Add<Tup<#full_rhs_generics>> for Tup<#full_generics>
                where #((#generics, #rhs_generics, #phantom_generics, #rhs_phantom_generics): crate::CanCombine),*,
                      #(#phantom_generics: core::default::Default, #rhs_phantom_generics: core::default::Default),*
            {
                type Output = Tup<
                    #(<(#generics, #rhs_generics, #phantom_generics, #rhs_phantom_generics) as crate::CanCombine>::Output),*,
                    #(<(#generics, #rhs_generics, #phantom_generics, #rhs_phantom_generics) as crate::CanCombine>::PhantomOutput),*>;

                fn add(self, rhs: Tup<#full_rhs_generics>) -> Self::Output{
                    Self::Output {
                        #(#fields: (self.#fields, rhs.#fields, <#phantom_generics as core::default::Default>::default(), <#rhs_phantom_generics as core::default::Default>::default()).combine()),*,
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
        result.extend(self.to_default_impl());
        result.extend(self.to_debug_impl());
        result.extend(self.to_add_impl());
        result
    }
}
