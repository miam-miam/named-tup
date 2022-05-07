use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{Token, TypeMacro};
use uuid::Uuid;

use crate::tup_element::{TupDefault, TupType};

#[derive(Default)]
pub struct TupDefaultReplace {
    pub struct_invocations: TokenStream,
}

impl TupDefaultReplace {
    fn produce_expr_struct(&mut self, elem: &mut TupType) {
        if let TupDefault::Unfinished(expr) = &elem.default {
            let struct_name = format_ident!("__{}_{}", elem.name, Uuid::new_v4().as_u128());
            let expr_type = &elem.value;
            let struct_tokens = quote! {
                #[allow(non_camel_case_types)]
                #[derive(Default, Copy, Clone)]
                struct #struct_name;
                impl named_tup::__private::TupDefault for #struct_name {
                    type Output = #expr_type;

                    fn default() -> Self::Output {
                        #expr
                    }
                }
            };
            self.struct_invocations.extend(struct_tokens);
            elem.default = TupDefault::Finished(struct_name);
        }
    }
}

impl VisitMut for TupDefaultReplace {
    fn visit_type_macro_mut(&mut self, i: &mut TypeMacro) {
        if i.mac.path.is_ident(&Ident::new("Tup", Span::call_site())) {
            let parser = Punctuated::<TupType, Token![,]>::parse_terminated;
            if let Ok(mut v) = parser.parse2(i.mac.tokens.clone()) {
                let new_expr = v.iter_mut().map(|elem| {
                    self.produce_expr_struct(elem);
                    elem.to_token_stream()
                });
                i.mac.tokens = quote!(#(#new_expr),*);
            };
        }
    }
}
