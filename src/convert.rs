use crate::tup_struct::{TupDefault, Unused, Used};

named_tup_derive::sealed_trait_builder!();

/// A copy of the [`Into`] trait from the standard library. The [`Into`] trait could unfortunately
/// not be used due to it's type reflexivity which clashed with the Tup implementation.
///
/// This trait is sealed as it should only ever be implemented on the Tup type.
///
/// A Tup can be transformed to another type if for all arguments it falls
/// in one of the following categories:
///
/// ```rust
/// # use named_tup::{TupInto, tup, tup_default};
/// #[tup_default]
/// pub fn main() {
///     let unused_to_unused: tup!(bar: () = ()) = tup!().into_tup();
///     assert_eq!(unused_to_unused, tup!());
///
///     let used_to_used: tup!(foo: i32) = tup!(foo: 3).into_tup();
///     assert_eq!(used_to_used, tup!(foo: 3));
///
///     let used_to_default: tup!(foo: &'static str = "hi") = tup!(foo: "yum").into_tup();
///     assert_eq!(used_to_default, tup!(foo: "yum"));
///
///     let unused_to_default: tup!(foo: f64 = 1.3) = tup!().into_tup();
///     assert_eq!(unused_to_default, tup!(foo: 1.3));
///
///     let default_to_used: tup!(foo: f64) = unused_to_default.into_tup();
///     assert_eq!(default_to_used, tup!(foo: 1.3));
/// }
/// ```
///
/// <br>
///
/// Since each rule acts individually on the tup's arguments we can combine them together.
///
/// ```rust
/// # use named_tup::{TupInto, tup, tup_default};
/// let colour = tup!(red: 65, green: 105, blue: 225);
/// let pixel = tup!(x: 5.0, y: 6.4, height: 4.7);
///
/// paint(pixel.into_tup(), colour.into_tup());
///
/// #[tup_default]
/// fn paint(pixel: tup!(x: f64, y: f64, height: f64 = 1.0, width: f64 = 1.0),
///          colour: tup!(red: i32, green: i32, blue: i32, opacity: f64 = 1.0))
/// {
///     let pixel_colour = pixel + colour;
///     // Paint    
/// }
/// ```
///
pub trait TupInto<T>: private::Sealed {
    /// Performs the conversion.
    #[must_use]
    fn into_tup(self) -> T;
}

/// A copy of the [`From`] trait from the standard library. The [`From`] trait could unfortunately
/// not be used due to it's type reflexivity which clashed with the Tup implementation.
///
/// This trait is sealed as it should only ever be implemented on the Tup type.
///
/// For more information please look at the [`TupInto`] trait.
/// ```rust
/// # use named_tup::{TupFrom, tup, tup_default};
/// #[tup_default]
/// pub fn main() {
///     let rick = tup!(funny: true);
///     let rick = <tup!(funny: bool, lyrics: &'static str = "Never Gonna Give You Up...")>::from_tup(rick);
///     assert_eq!(rick, tup!(funny: true, lyrics: "Never Gonna Give You Up..."))
/// }
/// ```
pub trait TupFrom<T>: private::Sealed {
    /// Performs the conversion
    #[must_use]
    fn from_tup(_: T) -> Self;
}

impl<T, U> TupInto<U> for T
where
    U: TupFrom<T>,
    T: private::Sealed,
{
    fn into_tup(self) -> U {
        U::from_tup(self)
    }
}

/// A helper trait to figure out if a tup field can transformed.
pub trait CanInto<OLD, NEW> {
    type Output;
    fn into(self) -> Self::Output;
}

impl CanInto<Unused, Unused> for () {
    type Output = ();
    fn into(self) {}
}

impl<T> CanInto<Used, Used> for T {
    type Output = T;
    fn into(self) -> T {
        self
    }
}

impl<D: TupDefault> CanInto<Unused, D> for () {
    type Output = D::Output;
    fn into(self) -> D::Output {
        D::default()
    }
}

impl<T, D> CanInto<Used, D> for T
where
    D: TupDefault<Output = T>,
{
    type Output = T;
    fn into(self) -> T {
        self
    }
}

impl<T, D> CanInto<D, Used> for T
where
    D: TupDefault<Output = T>,
{
    type Output = T;
    fn into(self) -> T {
        self
    }
}
