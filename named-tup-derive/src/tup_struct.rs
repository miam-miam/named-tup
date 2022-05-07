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
        let fields: Vec<Ident> = IDENTIFIERS
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
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let expanded = quote! {
            #[derive(Copy, Clone)]
            #[must_use]
            #[allow(clippy::type_complexity)]
            pub struct Tup<#full_generics> {
                #(pub #fields: #generics,)*
                _phantom: core::marker::PhantomData<(#(#phantom_generics),*)>
            }
        };

        expanded
    }

    fn to_new_impl(&self) -> TokenStream {
        let Self {
            generics,
            fields,
            full_generics,
            ..
        } = self;

        let expanded = quote! {
            impl<#full_generics> Tup<#full_generics> {
                #[allow(clippy::too_many_arguments)]
                pub fn new(#(#fields: #generics),*) -> Self {
                    Tup {
                        #(#fields,)*
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
            impl core::default::Default for Tup<#(#generics,)* #(#phantom_generics),*> {
                fn default() -> Self {
                    Tup {
                        #(#fields: (),)*
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        expanded
    }

    fn to_debug_impl(&self) -> TokenStream {
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let where_clause = match fields.is_empty() {
            true => quote! {},
            false => quote! {
                where #(#phantom_generics: crate::tup_struct::ConvertToDebugStruct + core::default::Default),*,
                #(#generics: core::fmt::Debug),*
            },
        };

        let expanded = quote! {
            impl<#full_generics> core::fmt::Debug for Tup<#full_generics>
                #where_clause
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
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let coma = match fields.is_empty() {
            true => quote! {},
            false => quote! {,},
        };

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

        let where_clause = match fields.is_empty() {
            true => quote! {},
            false => quote! {
                where #((#generics, #rhs_generics): crate::combine::CanCombine<#phantom_generics, #rhs_phantom_generics>),*
            },
        };

        let expanded = quote! {
            impl<#full_generics #coma #full_rhs_generics> core::ops::Add<Tup<#full_rhs_generics>> for Tup<#full_generics>
                #where_clause
            {
                type Output = Tup<
                    #(<(#generics, #rhs_generics) as crate::combine::CanCombine<#phantom_generics, #rhs_phantom_generics>>::Output),* #coma
                    #(<(#generics, #rhs_generics) as crate::combine::CanCombine<#phantom_generics, #rhs_phantom_generics>>::PhantomOutput),*>;

                fn add(self, rhs: Tup<#full_rhs_generics>) -> Self::Output{
                    Self::Output {
                        #(#fields: crate::combine::CanCombine::<#phantom_generics, #rhs_phantom_generics>::combine((self.#fields, rhs.#fields)) ),* #coma
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        expanded
    }

    fn to_into_impl(&self) -> TokenStream {
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let coma = match fields.is_empty() {
            true => quote! {},
            false => quote! {,},
        };

        let new_phantom_generics_stored: Vec<Ident> = phantom_generics
            .iter()
            .map(|g| format_ident!("NEW{g}"))
            .collect();
        let new_phantom_generics = &new_phantom_generics_stored;

        let where_clause = match fields.is_empty() {
            true => quote! {},
            false => quote! {
                where #(#generics: crate::convert::CanInto<#phantom_generics, #new_phantom_generics>),*
            },
        };

        let expanded = quote! {
            impl<#full_generics #coma #(#new_phantom_generics),*> crate::convert::TupFrom<Tup<#full_generics>> for Tup<#(<#generics as crate::convert::CanInto<#phantom_generics, #new_phantom_generics>>::Output,)* #(#new_phantom_generics),*>
                #where_clause
            {
                fn from_tup(current: Tup<#full_generics>) -> Self {
                    Self {
                        #(#fields: crate::convert::CanInto::<#phantom_generics, #new_phantom_generics>::into(current.#fields) ,)*
                        _phantom: core::marker::PhantomData
                    }
                }
            }
        };

        expanded
    }

    fn to_eq_impl(&self) -> TokenStream {
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let rhs_phantom_generics_stored: Vec<Ident> = phantom_generics
            .iter()
            .map(|g| format_ident!("RHS{g}"))
            .collect();
        let rhs_phantom_generics = &rhs_phantom_generics_stored;

        let generics_rhs_phantom = quote! {#(#generics,)* #(#rhs_phantom_generics),*};
        let all_phantom = quote! {#(#phantom_generics,)* #(#rhs_phantom_generics),*};

        let return_stmt_eq = match self.fields.is_empty() {
            true => quote! {true},
            false => quote! {#((&self.#fields) == (&other.#fields))&&*},
        };

        let expanded = quote! {
            impl<#(#generics: core::cmp::Eq,)* #(#phantom_generics),*> core::cmp::Eq for Tup<#full_generics> {}

            impl<#(#generics: core::cmp::PartialEq,)* #all_phantom> core::cmp::PartialEq<Tup<#generics_rhs_phantom>> for Tup<#full_generics>
            {
                fn eq(&self, other: &Tup<#generics_rhs_phantom>) -> bool {
                    #return_stmt_eq
                }
            }
        };
        expanded
    }

    // Cannot implement true ord as that requires the same type.
    fn to_ord_impl(&self) -> TokenStream {
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let rhs_phantom_generics_stored: Vec<Ident> = phantom_generics
            .iter()
            .map(|g| format_ident!("RHS{g}"))
            .collect();
        let rhs_phantom_generics = &rhs_phantom_generics_stored;

        let generics_rhs_phantom = quote! {#(#generics,)* #(#rhs_phantom_generics),*};
        let all_phantom = quote! {#(#phantom_generics,)* #(#rhs_phantom_generics),*};

        let mut return_stmt_ord = quote! {core::cmp::Ordering::Equal};
        for field in fields.iter().rev() {
            return_stmt_ord = quote! {
                match core::cmp::Ord::cmp(&self.#field, &other.#field) {
                    core::cmp::Ordering::Equal => {
                        #return_stmt_ord
                    },
                    cmp => cmp,
                }
            };
        }

        let mut return_stmt_part = quote! {core::option::Option::Some(core::cmp::Ordering::Equal)};
        for field in fields.iter().rev() {
            return_stmt_part = quote! {
                match ::core::cmp::PartialOrd::partial_cmp(&self.#field, &other.#field) {
                    core::option::Option::Some(core::cmp::Ordering::Equal) => {
                        #return_stmt_part
                    },
                    cmp => cmp,
                }
            };
        }

        let expanded = quote! {
            impl<#(#generics: core::cmp::Ord,)* #(#phantom_generics),*> core::cmp::Ord for Tup<#full_generics> {
                fn cmp(&self, other: &Tup<#full_generics>) -> core::cmp::Ordering {
                    #return_stmt_ord
                }
            }

            impl<#(#generics: core::cmp::PartialOrd,)* #all_phantom> core::cmp::PartialOrd<Tup<#generics_rhs_phantom>> for Tup<#full_generics> {
                fn partial_cmp(&self, other: &Tup<#generics_rhs_phantom>) -> core::option::Option<core::cmp::Ordering> {
                    #return_stmt_part
                }
            }
        };
        expanded
    }

    fn to_hash_impl(&self) -> TokenStream {
        let Self {
            generics,
            fields,
            phantom_generics,
            full_generics,
        } = self;

        let expanded = quote! {
            impl<#(#generics: core::hash::Hash,)* #(#phantom_generics),*> core::hash::Hash for Tup<#full_generics>
            {
                fn hash<__H: core::hash::Hasher>(&self, state: &mut __H) {
                    #(core::hash::Hash::hash(&self.#fields, state));*
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
        result.extend(self.to_eq_impl());
        result.extend(self.to_ord_impl());
        result.extend(self.to_hash_impl());
        result
    }
}
