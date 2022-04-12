#![allow(dead_code)]

use core::fmt::{Debug, DebugStruct};

use named_tup_derive;

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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NotUnit;

pub trait CanCombine {
    type Output;
    type PhantomOutput;
    fn combine(self) -> Self::Output;
}

impl<T> CanCombine for (T, T, NotUnit, NotUnit) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.1
    }
}

impl<T> CanCombine for (T, (), NotUnit, ()) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.0
    }
}

impl<T> CanCombine for ((), T, (), NotUnit) {
    type Output = T;
    type PhantomOutput = NotUnit;
    fn combine(self) -> T {
        self.1
    }
}

impl CanCombine for ((), (), (), ()) {
    type Output = ();
    type PhantomOutput = ();
    fn combine(self) -> () {
        ()
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

impl ConvertToDebugStruct for () {
    fn convert(_: Self, _debug_struct: &mut DebugStruct, _name: &str, _value: &dyn Debug) {}
}
