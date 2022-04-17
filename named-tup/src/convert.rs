use core::marker::Sized;

use crate::tup_struct;
use crate::tup_struct::{TupDefault, Unused, Used};

/// new to create empty tuple
/// each named argument creates a separate struct like a builder struct where they each implement
/// a Trait with setter for each named argument they don't have
/// a Trait with getter+setter for each named argument they do have
/// a new Trait is created that is based on Default so that the struct can into in
/// add type based tup! so that it can be used in functions
/// The supported operations on tuples would be accessing member variables,
/// + (only if there is no intersection), - and adding.
/// Should allow you to do tup!(foo: 5, bar) where bar is an already defined var.
/// Since everything is a struct we get named-tup-derive traits for free!

pub trait TupInto<T> {
    #[must_use]
    fn into_tup(self) -> T;
}

//TODO: Generate Sealed Traits
pub trait TupFrom<T> {
    #[must_use]
    fn from_tup(_: T) -> Self;
}

impl<T, U> TupInto<U> for T
where
    U: TupFrom<T>,
{
    fn into_tup(self) -> U {
        U::from_tup(self)
    }
}

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
