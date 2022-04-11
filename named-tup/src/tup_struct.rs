#![allow(dead_code)]

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

pub struct NotUnit;

pub trait CanCombine {
    type Output;
    fn combine(self) -> Self::Output;
}

impl<T> CanCombine for (T, T, NotUnit, NotUnit) {
    type Output = T;

    fn combine(self) -> T {
        self.0
    }
}

impl<T> CanCombine for (T, (), NotUnit, ()) {
    type Output = T;

    fn combine(self) -> T {
        self.0
    }
}

impl<T> CanCombine for ((), T, (), NotUnit) {
    type Output = T;

    fn combine(self) -> T {
        self.1
    }
}

impl CanCombine for ((), (), (), ()) {
    type Output = ();

    fn combine(self) -> () {
        ()
    }
}

// Could have return type in trait.

// impl Tup {
//     fn add<LT1, LT2, LT3, LA1, LA2, LA3, RT1, RT2, RT3, RA1, RA2, RA3>(lhs: Tup<LT1, LT2, LT3, LA1, LA2, LA3>, rhs: Tup<RT1, RT2, RT3, RA1, RA2, RA3>)
//         where (LT1, RT1, LA1, RA1) : CanAdd
//         ...
//     {
//          lhs.add_1(rhs).add_2(rhs).add_3(rhs)
//     }
// }

struct Tupfoo<T> {
    foo: T,
}
