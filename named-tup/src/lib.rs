extern crate core;

pub use named_tup_derive::tup;
pub use tup_struct::{CanCombine, ConvertToDebugStruct, NotUnit, Tup};

mod tup_struct;
