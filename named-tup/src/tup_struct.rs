use core::fmt::{Debug, DebugStruct};

named_tup_derive::tup_struct_builder!();

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Used;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Unused;

pub trait TupDefault {
    type Output;
    fn default() -> Self::Output;
}

pub trait ConvertToDebugStruct {
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug);
}

impl ConvertToDebugStruct for Used {
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

impl ConvertToDebugStruct for Unused {
    fn convert(_: Self, _debug_struct: &mut DebugStruct, _name: &str, _value: &dyn Debug) {}
}
