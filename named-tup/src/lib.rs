#![no_std]
//! [![github]](https://github.com/miam-miam100/named-tup)&ensp;[![crates-io]](https://crates.io/crates/named-tup)&ensp;[![docs-rs]](https://docs.rs/named-tup)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This crate provides the [`tup!`] macro to produce named tuples, a struct that
//! can contain a set of named arguments. Each named tuple can be added together or even
//! default to a value if it does not already exist.
//!
//!
//! The idea of named tuples is to provide a way to quickly iterate on ideas without having
//! to create a builder struct or losing the ability to type check at compile time.
//! Named tuples also allow the creation of default values that can replace nonexistent arguments.
//!
//! ```toml
//! [dependencies]
//! named-tup = "0.1.0"
//!
//! [package.metadata.inwelling]
//! named-tup-derive = true
//! ```
//!
//! <br>
//!
//! # Examples
//! ```
//! use named_tup::tup;
//!
//! let count = 5;
//! // This will have the type of tup!(count: i32, ingredients: [&str; 3], eggs: bool)
//! let cakes = tup!(count, ingredients: ["milk", "flower", "sugar"], eggs: true);
//! // We can just add a price afterwards
//! // And now it has the type of tup!(eggs: bool, ingredients: [&str; 3], count: i32, price: i32)
//! let mut cakes = cakes + tup!(price: 3);
//! // Once the price is in the tup we can just update it!
//! cakes.price = 4;
//! // Will print tup { count: 5, eggs: true, ingredients: ["milk", "flower", "sugar"], price: 4 }
//! println!("{cakes:?}");
//! ```
//!
//! <br>
//!
//! To use defaults just annotate the item where you set a field with [`#[tup_default]`](tup_default).
//! Additionally since the defaulted [`tup!`] is a type you need to convert into it by calling
//! [`.into_tup()`](TupInto) which can be accessed through the [`TupInto`] trait.
//!
//! ```
//! use named_tup::{tup, tup_default, TupInto};
//!
//! let options = tup!(read: false, write: true);
//! // Converts to tup!(read: false, write: true, create: false, timeout: 5)
//! open_file("main.rs", options.into_tup());
//!
//! #[tup_default]
//! fn open_file(
//!     path: &str,
//!     options: tup!(
//!         read: bool = true,
//!         write: bool = false,
//!         create: bool = false,
//!         timeout: i32 = 5
//!     ))
//! {
//!     // Open the file
//! }
//! ```
// Tup types in rustdoc of other crates get linked to here.
#![doc(html_root_url = "https://docs.rs/named_tup/0.1.0")]

pub use convert::{TupFrom, TupInto};
/// Test
pub use named_tup_derive::tup;
/// Test2
pub use named_tup_derive::tup_default;

mod combine;
mod convert;
mod tup_struct;

//Not part of public api.
#[doc(hidden)]
pub mod __private {
    pub use super::tup_struct::{Tup, TupDefault, Unused, Used};
}
