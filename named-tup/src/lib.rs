extern crate core;

pub use named_tup_derive::{tup, tup_default};
pub use tup_struct::{
    CanCombine, CanInto, ConvertToDebugStruct, NotUnit, Tup, TupDefault, TupFrom, TupInto,
};

mod tup_struct;
