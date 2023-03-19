use std::collections::HashSet;
use std::fs;
use std::path::Path;

use syn::parse::{Parse, ParseStream};
use syn::visit::Visit;
use syn::{visit, Macro, PathArguments, Result, Token};

mod tup_element {
    include! {"src/tup_element.rs"}
}

struct TupElementInvocation(Vec<tup_element::TupElement>);

struct TupTypeInvocation(Vec<tup_element::TupType>);

impl Parse for TupElementInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values: Vec<tup_element::TupElement> = input
            .parse_terminated(tup_element::TupElement::parse, Token![,])?
            .into_iter()
            .collect();
        values.sort();
        Ok(TupElementInvocation(values))
    }
}

impl Parse for TupTypeInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values: Vec<tup_element::TupType> = input
            .parse_terminated(tup_element::TupType::parse, Token![,])?
            .into_iter()
            .collect();
        values.sort();
        Ok(TupTypeInvocation(values))
    }
}

struct TupFinder<'a>(&'a mut HashSet<String>);

impl<'a> Visit<'_> for TupFinder<'a> {
    fn visit_macro(&mut self, mac: &Macro) {
        if mac.path.leading_colon.is_none() && mac.path.segments.len() == 1 {
            let seg = mac.path.segments.first().unwrap();
            if let PathArguments::None = seg.arguments {
                if seg.ident == "tup" {
                    let mac = syn::parse2::<TupElementInvocation>(mac.tokens.clone());
                    if let Ok(mac) = mac {
                        for tuple in mac.0 {
                            let ident = tuple.name.to_string();
                            self.0.insert(ident);
                        }
                    }
                } else if seg.ident == "Tup" {
                    let mac = syn::parse2::<TupTypeInvocation>(mac.tokens.clone());

                    if let Ok(mac) = mac {
                        for tuple in mac.0 {
                            let ident = tuple.name.to_string();
                            self.0.insert(ident);
                        }
                    }
                }
            }
        }
        visit::visit_macro(self, mac);
    }
}

pub fn get_all_identifiers(file_path: &Path, all_identifiers: &mut HashSet<String>) {
    let code = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Could not read to file at: {file_path:?}"));

    let syntax = syn::parse_file(&code);
    if let Ok(syntax) = syntax {
        TupFinder(all_identifiers).visit_file(&syntax);
    }
}
