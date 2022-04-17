#![allow(dead_code)]

use core::fmt::{Debug, DebugStruct};

named_tup_derive::tup_struct_builder!();
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

pub trait TupFrom<T>: Sized {
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NotUnit;

pub trait TupDefault {
    type Output;
    fn default() -> Self::Output;
}

pub trait CanInto<OLD, NEW> {
    type Output;
    fn into(self) -> Self::Output;
}

impl CanInto<(), ()> for () {
    type Output = ();
    fn into(self) {}
}

impl<T> CanInto<NotUnit, NotUnit> for T {
    type Output = T;
    fn into(self) -> T {
        self
    }
}

impl<D: TupDefault> CanInto<(), D> for () {
    type Output = D::Output;
    fn into(self) -> D::Output {
        D::default()
    }
}

impl<T, D> CanInto<NotUnit, D> for T
where
    D: TupDefault<Output = T>,
{
    type Output = T;
    fn into(self) -> T {
        self
    }
}

impl<T, D> CanInto<D, NotUnit> for T
where
    D: TupDefault<Output = T>,
{
    type Output = T;
    fn into(self) -> T {
        self
    }
}

pub trait CanCombine<P1, P2> {
    type Output;
    type PhantomOutput;
    fn combine(self) -> Self::Output;
}

impl<T> CanCombine<NotUnit, NotUnit> for (T, T) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.1
    }
}

impl<T> CanCombine<NotUnit, ()> for (T, ()) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.0
    }
}

impl<T> CanCombine<(), NotUnit> for ((), T) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.1
    }
}

impl CanCombine<(), ()> for ((), ()) {
    type Output = ();
    type PhantomOutput = ();
    fn combine(self) {}
}

impl<T, D: TupDefault> CanCombine<NotUnit, D> for (T, T) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.0
    }
}

impl<T, D1: TupDefault, D2: TupDefault> CanCombine<D1, D2> for (T, T) {
    type Output = T;
    type PhantomOutput = D2;
    fn combine(self) -> T {
        self.1
    }
}

impl<T, D: TupDefault> CanCombine<D, NotUnit> for (T, T) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.1
    }
}

impl<T, D: TupDefault> CanCombine<(), D> for ((), T) {
    type Output = T;
    type PhantomOutput = D;
    fn combine(self) -> T {
        self.1
    }
}

impl<T, D: TupDefault> CanCombine<D, ()> for (T, ()) {
    type Output = T;
    type PhantomOutput = D;
    fn combine(self) -> T {
        self.0
    }
}

pub trait ConvertToDebugStruct {
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug);
}

impl ConvertToDebugStruct for NotUnit {
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug) {
        debug_struct.field(name, value);
    }
}

//TODO: Make a nice Debug for default.
impl<T: TupDefault> ConvertToDebugStruct for T {
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug) {
        debug_struct.field(name, value);
    }
}

impl ConvertToDebugStruct for () {
    fn convert(_: Self, _debug_struct: &mut DebugStruct, _name: &str, _value: &dyn Debug) {}
}
