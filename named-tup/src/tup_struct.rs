use core::fmt::{Debug, DebugStruct, Formatter};

named_tup_derive::tup_struct_builder!();

/// A Unit Struct indicating that an argument has been set
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Used;

/// A Unit Struct indicating that an argument has not been set
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Unused;

/// A trait that is implemented by a unit struct that generates a default value
pub trait TupDefault {
    type Output;
    fn default() -> Self::Output;
}

/// A trait that allows to convert a Tup to a Debug version depending on the phantom type.
pub trait ConvertToDebugStruct {
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug);
}

impl ConvertToDebugStruct for Used {
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug) {
        debug_struct.field(name, value);
    }
}

/// A new type used to hijack the Debug implementation for the debug_struct
struct DebugHijacker<'a>(&'a dyn Debug, &'a dyn Debug);

impl<'a> Debug for DebugHijacker<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)?;
        f.write_str(" = ")?;
        self.1.fmt(f)
    }
}

impl<T: TupDefault> ConvertToDebugStruct for T
where
    T::Output: Debug,
{
    fn convert(_: Self, debug_struct: &mut DebugStruct, name: &str, value: &dyn Debug) {
        debug_struct.field(name, &DebugHijacker(value, &T::default()));
    }
}

impl ConvertToDebugStruct for Unused {
    fn convert(_: Self, _debug_struct: &mut DebugStruct, _name: &str, _value: &dyn Debug) {}
}
