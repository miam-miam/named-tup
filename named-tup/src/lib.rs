#![no_std]

pub use convert::{TupFrom, TupInto};
pub use named_tup_derive::{tup, tup_default};

mod combine;
mod convert;
mod tup_struct;

//Not part of public api.
#[doc(hidden)]
pub mod __private {
    pub use super::tup_struct::{Tup, TupDefault, Unused, Used};
}
