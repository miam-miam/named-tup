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

        let expanded = quote! {
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[must_use]
            pub struct Tup<#full_generics> {
                #(pub #fields: #generics),*,
                _phantom: core::marker::PhantomData<(#(#phantom_generics),*)>
            }
        };

        expanded
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

        expanded
    }

    fn to_default_impl(&self) -> TokenStream {
        let fields = &self.fields;
        let generics = (0..self.generics.len()).map(|_| syn::parse_str::<syn::Type>("()").unwrap());
        let phantom_generics = (0..self.phantom_generics.len())
            .map(|_| syn::parse_str::<syn::Type>("crate::tup_struct::Unused").unwrap());

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

        expanded
    }

    fn to_debug_impl(&self) -> TokenStream {
        let (generics, fields, full_generics, phantom_generics) = (
            &self.generics,
            &self.fields,
            &self.full_generics,
            &self.phantom_generics,
        );

        let expanded = quote! {
            impl<#full_generics> core::fmt::Debug for Tup<#full_generics>
                where #(#phantom_generics: crate::tup_struct::ConvertToDebugStruct + core::default::Default),*,
                #(#generics: core::fmt::Debug),*
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    let mut debug_struct = f.debug_struct("tup");
                    #(crate::tup_struct::ConvertToDebugStruct::convert(<#phantom_generics as core::default::Default>::default(), &mut debug_struct, stringify!(#fields), &self.#fields);)*
                    debug_struct.finish()
                }
            }
        };

        expanded
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
                where #((#generics, #rhs_generics): crate::combine::CanCombine<#phantom_generics, #rhs_phantom_generics>),*
            {
                type Output = Tup<
                    #(<(#generics, #rhs_generics) as crate::combine::CanCombine<#phantom_generics, #rhs_phantom_generics>>::Output),*,
                    #(<(#generics, #rhs_generics) as crate::combine::CanCombine<#phantom_generics, #rhs_phantom_generics>>::PhantomOutput),*>;

                fn add(self, rhs: Tup<#full_rhs_generics>) -> Self::Output{
                    Self::Output {
                        #(#fields: crate::combine::CanCombine::<#phantom_generics, #rhs_phantom_generics>::combine((self.#fields, rhs.#fields)) ),*,
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        expanded
    }

    fn to_into_impl(&self) -> TokenStream {
        let (generics, fields, full_generics, phantom_generics) = (
            &self.generics,
            &self.fields,
            &self.full_generics,
            &self.phantom_generics,
        );

        let new_phantom_generics_stored: Vec<Ident> = phantom_generics
            .iter()
            .map(|g| format_ident!("NEW{g}"))
            .collect();
        let new_phantom_generics = &new_phantom_generics_stored;

        // From<>
        let expanded = quote! {
            impl<#full_generics, #(#new_phantom_generics),*> crate::convert::TupFrom<Tup<#full_generics>> for Tup<#(<#generics as crate::convert::CanInto<#phantom_generics, #new_phantom_generics>>::Output),*, #(#new_phantom_generics),*>
                where #(#generics: crate::convert::CanInto<#phantom_generics, #new_phantom_generics>),*
            {
                //(), (), (), (), (), ()
                //Self = (), i32, (), (), __count_33453526383602638678589360769295210241, ()
                fn from_tup(current: Tup<#full_generics>) -> Self {
                    Self {
                        #(#fields: crate::convert::CanInto::<#phantom_generics, #new_phantom_generics>::into(current.#fields) ),*,
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        expanded
    }

    pub fn to_token_stream(&self) -> TokenStream {
        let mut result = self.to_def();
        result.extend(self.to_new_impl());
        result.extend(self.to_default_impl());
        result.extend(self.to_debug_impl());
        result.extend(self.to_add_impl());
        result.extend(self.to_into_impl());
        result
    }
}
