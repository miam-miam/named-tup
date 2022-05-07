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
//! named-tup = "0.1.1"
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
//! let count = 5;
//!
//! // This will have the type of Tup!(count: i32, ingredients: [&str; 3], eggs: bool)
//! let cakes = tup!(count, ingredients: ["milk", "flower", "sugar"], eggs: true);
//!
//! // We can just add a price afterwards
//! let mut cakes = cakes + tup!(price: 3);
//! // And now it has the type of Tup!(eggs: bool, ingredients: [&str; 3], count: i32, price: i32)
//!
//! // Once the price is in the tup we can just update it!
//! cakes.price = 4;
//!
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
//! use named_tup::{tup,Tup, tup_default, TupInto};
//!
//! let options = tup!(read: false, write: true);
//!
//! // Converts to Tup!(read: false, write: true, create: false, timeout: 5)
//! open_file("main.rs", options.into_tup());
//!
//! #[tup_default]
//! fn open_file(
//!     path: &str,
//!     options: Tup!(
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
#![doc(html_root_url = "https://docs.rs/named_tup/0.1.1")]

pub use convert::{TupFrom, TupInto};
/// The whole point.
///
/// Produces a named tuple, a struct that
/// can contain a set of named arguments. Each named tuple can be added together or even
/// default to a value if it does not already exist.
///
/// There are two types of call notations depending on whether the [`tup!`] macro is
/// used to define a type or an expression.
///
/// <br>
///
/// # tup as an expression
///
/// In it's most basic form a [`tup!`] is formed just like a struct instantiation.
/// ```rust
/// # use named_tup::tup;
/// let mut farm = tup!(horse: 4, chicken: 3, ants: 999_999_999);
///
/// assert_eq!(farm.horse, 4);
/// assert_eq!(farm.chicken, 3);
///
/// // Got some ant killer
/// farm.ants = 0;
/// assert_eq!(farm.ants, 0);
/// ```
///
/// And just like it, [`tup!`] can use pre-existing variables.
/// ```rust
/// # use named_tup::tup;
/// let kingfisher = true;
/// let eagle = false;
/// let nest = tup!(kingfisher, toucan: true, eagle);
///
/// assert_eq!(nest, tup!(kingfisher: true, eagle: false, toucan: true))
/// ```
///
/// <br>
///
/// # tup as a type
///
/// However in certain cases defining the type exactly is necessary.
/// In this case the expression is replaced by a type.
///
/// ```rust
/// # use named_tup::{tup, Tup};
/// let recipe: Tup!(eggs: u8, milk: &str, flour: f32) = tup!(milk: "500ml", eggs: 4, flour: 203.6);
/// let person = tup!(name: "Joe", blue_eyes: true);
/// face_recognizer(vec![person]);
///
/// fn face_recognizer(
///     people: Vec<Tup!(name: &'static str, blue_eyes: bool)>,
/// ) -> Tup!(confidence: f64, name: &'static str) {
///     tup!(confidence: 0.3, name: "Joe")
/// }
/// ```
///
/// The type macro is also used to specify defaults using the [`#[tup_default]`](tup_default)
/// attribute and the [`TupInto`] trait to change the type.
///
/// ```rust
/// # use named_tup::{tup, Tup, tup_default, TupInto};
/// #[tup_default]
/// pub fn main() {
///     # let input = false;
///     let result: Tup!(foo: i32 = 3, bar: Option<i32> = None) = match input {
///         true => tup!(foo: 4).into_tup(),
///         false => tup!(bar: Some(4)).into_tup(),
///     };
///     
///     read(tup!().into_tup());
/// }
///
/// #[tup_default]
/// fn read(books: Tup!(names: &'static [&'static str] = &[], ETA: i32 = 0)) {
///     // Read
/// }
/// ```
///
/// <br>
///
/// # Tup type
///
/// Each [`tup!`] call produces a Tup type, the type itself eagerly implements
/// [`Copy`], [`Clone`], [`Eq`], [`PartialEq`], [`Ord`], [`PartialOrd`], [`Hash`]
/// assuming all the types it contains implement them. (Ord/PartialOrd is in lexicographic
/// ordering and Ord/Eq cannot be implemented on types that use different defaults
/// so if this is the case just convert them to non-defaulted versions before using them).
/// As well as this a [`Default`] and [`Debug`] trait is always implemented.
///
/// ```rust
/// # use named_tup::tup;
/// assert_eq!(tup!(rooms: ["garden", "shed"]), tup!(rooms: ["garden", "shed"]));
///
/// let rooms = vec!["bathroom", "bedroom"];
/// let non_copy = tup!(rooms);
/// assert_eq!(non_copy, non_copy.clone());
///
/// let copy = tup!(cows: 4, bulls: 2);
/// drop(copy);
/// assert!(copy > tup!(cows: 5, bulls: 1));
///
/// // Will print tup { farmer: "Joe", married: true }
/// println!("{:?}", tup!( married: true, farmer: "Joe"));
/// ```
///
/// Finally the [`Add`](core::ops::Add) trait is implemented so that you can transform between
/// different tup types. If both sides contain a certain argument, precedence is given to the
/// right hand side.
/// ```rust
/// # use named_tup::tup;
/// let farm1 = tup!(roosters: 4, dragons: 7, dogs: 1);
/// let farm2 = tup!(hens: 56, dogs: 3);
/// let combined_farm = farm1 + farm2;
///
/// assert_eq!(combined_farm, tup!(roosters: 4, hens: 56, dragons: 7, dogs: 3));
/// ```
pub use named_tup_derive::tup;
/// An attribute macro that allows you to derive defaults.
///
/// Defaults are added to any [`tup!`] macro in the type position by using the equals sign.
/// [`#[tup_default]`](tup_default) will then change the invocation so that it is a part of the
/// type information itself. As such [`#[tup_default]`](tup_default) needs to be used on any
/// item that uses defaults in a [`tup!`] invocation. Since a defaulted Tup is part of
/// the type [`TupInto`] must be used to convert it.
///
/// ```rust
/// # use named_tup::{TupInto, tup, tup_default, Tup};
/// #[tup_default]
/// pub fn main() {
///     let default: Tup!(foo: i32 = 2) = tup!().into_tup();
///     let result = default_to_non(tup!().into_tup());
///
///     assert_eq!(result, tup!(foo: 2));
///     assert_eq!(result.foo, default.foo);
/// }
/// #[tup_default]
/// fn default_to_non(n_tup: Tup!(foo: i32 = 2)) -> Tup!(foo: i32) {
///     n_tup.into_tup()
/// }
///
/// ```
pub use named_tup_derive::tup_default;
///
///
pub use named_tup_derive::Tup;

mod combine;
mod convert;
mod tup_struct;

//Not part of public api.
#[doc(hidden)]
pub mod __private {
    pub use super::tup_struct::{Tup, TupDefault, Unused, Used};
}