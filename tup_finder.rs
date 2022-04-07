use pest::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(pest_derive::Parser)]
#[grammar = "../grammar.pest"]
struct TupFinder;

pub fn get_all_identifiers(files: Vec<PathBuf>) -> Vec<String> {
    let mut all_identifiers = HashSet::new();
    dbg!(&files);
    for file in files {
        let code = fs::read_to_string(file).unwrap();

        let tuples = TupFinder::parse(Rule::file, &code);

        if let Ok(tuples) = tuples {
            for tuple in tuples {
                let ident = tuple.as_str();
                all_identifiers.insert(ident.to_string());
            }
        }
    }
    let mut all_identifiers: Vec<String> = all_identifiers.into_iter().collect();
    all_identifiers.sort();
    all_identifiers
}
