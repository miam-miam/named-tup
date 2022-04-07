pub const IDENTIFIERS: &'static [&'static str] =
    include!(concat!(env!("OUT_DIR"), "/identifiers.in"));
mod tup;
