use std::collections::HashSet;
use std::fs;
use std::path::Path;

use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "../grammar.pest"]
struct TupFinder;

pub fn get_all_identifiers(file_path: &Path, all_identifiers: &mut HashSet<String>) {
    let code = fs::read_to_string(file_path).unwrap();

    let tuples = TupFinder::parse(Rule::file, &code);

    if let Ok(tuples) = tuples {
        for tuple in tuples {
            let ident = tuple.as_str();
            all_identifiers.insert(ident.to_string());
        }
    }
}
