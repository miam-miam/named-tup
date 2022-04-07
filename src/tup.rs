#![allow(dead_code)]

struct Tup;
/// new to create empty tuple
/// each named argument creates a separate struct like a builder struct where they each implement
/// a Trait with setter for each named argument they don't have
/// a Trait with getter+setter for each named argument they do have
/// a new Trait is created that is based on Default so that the struct can into in
/// add type based tup! so that it can be used in functions
/// The supported operations on tuples would be accessing member variables,
/// + (only if there is no intersection), - and adding.
/// Should allow you to do tup!(foo: 5, bar) where bar is an already defined var.
/// Since everything is a struct we get derive traits for free!

struct Tupfoo<T> {
    foo: T,
}
