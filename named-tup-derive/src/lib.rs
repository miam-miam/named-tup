extern crate core;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use std::collections::HashSet;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

use once_cell::sync::Lazy;
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, Item};

use crate::tup_default::TupDefaultReplace;
use crate::tup_invocation::TupInvocation;

mod sealed;
mod tup_default;
mod tup_element;
mod tup_invocation;
mod tup_struct;

const IDENTIFIERS: &[&str] = include!(concat!(env!("OUT_DIR"), "/identifiers.in"));

struct Test {
    identifiers: HashSet<String>,
    file: File,
}

impl Test {
    pub fn new() -> Test {
        let mut old_identifiers = vec![];
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(concat!(env!("OUT_DIR"), "/identifiers.in"))
            .unwrap();
        Self::get_identifiers(&mut file, &mut old_identifiers);
        let identifiers = HashSet::from_iter(old_identifiers);
        Test { identifiers, file }
    }

    pub fn add_identifiers(&mut self, vals: impl Iterator<Item = String>) {
        let mut changed = false;
        for val in vals {
            changed = self.identifiers.insert(val) || changed;
        }
        if changed {
            let mut all_identifiers: Vec<String> = self.identifiers.clone().into_iter().collect();
            all_identifiers.sort();
            self.file.set_len(0).unwrap();
            self.file.seek(SeekFrom::Start(0)).unwrap();
            self.file
                .write_all(format!("&{all_identifiers:?}").as_ref())
                .unwrap();
        }
    }

    fn get_identifiers(file: &mut File, identifiers: &mut Vec<String>) {
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();
        let striped = file_content
            .strip_suffix("\r\n")
            .or_else(|| file_content.strip_suffix('\n'))
            .unwrap_or(&*file_content);
        let file_content = &striped[2..striped.len() - 1];
        identifiers.extend(
            file_content
                .split(", ")
                .filter(|s| s != &"")
                .map(|s| s[1..s.len() - 1].to_string()),
        );
    }
}

static NEW_IDENTIFIERS: Lazy<Mutex<Test>> = Lazy::new(|| {
    let m = Test::new();
    Mutex::new(m)
});

#[proc_macro]
pub fn tup_struct_builder(_input: TokenStream) -> TokenStream {
    TokenStream::from(tup_struct::TupInfo::new().to_token_stream())
}

#[proc_macro]
pub fn sealed_trait_builder(_input: TokenStream) -> TokenStream {
    TokenStream::from(sealed::to_token_stream())
}

#[proc_macro]
pub fn tup(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        return quote! {named_tup::__private::Tup::default()}.into();
    }
    let input = parse_macro_input!(input as TupInvocation);
    input.add_identifiers();
    TokenStream::from(input.into_token_stream())
}

#[proc_macro_attribute]
pub fn tup_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut syntax_tree = parse_macro_input!(item as Item);
    let mut replace = TupDefaultReplace::default();
    replace.visit_item_mut(&mut syntax_tree);
    let struct_invocations = replace.struct_invocations;
    let expanded = quote! {
        #struct_invocations
        #syntax_tree
    };
    expanded.into()
}
