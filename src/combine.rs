use crate::tup_struct::{TupDefault, Unused, Used};

/// Defines how two tup's arguments can be added together
pub trait CanCombine<P1, P2> {
    type Output;
    type PhantomOutput;
    fn combine(self) -> Self::Output;
}

impl<T> CanCombine<Used, Used> for (T, T) {
    type Output = T;
    type PhantomOutput = Used;
    fn combine(self) -> T {
        self.1
    }
}

impl<T> CanCombine<Used, Unused> for (T, ()) {
    type Output = T;
    type PhantomOutput = Used;
    fn combine(self) -> T {
        self.0
    }
}

impl<T> CanCombine<Unused, Used> for ((), T) {
    type Output = T;
    type PhantomOutput = Used;
    fn combine(self) -> T {
        self.1
    }
}

impl CanCombine<Unused, Unused> for ((), ()) {
    type Output = ();
    type PhantomOutput = Unused;
    fn combine(self) {}
}

impl<T, D: TupDefault> CanCombine<Used, D> for (T, T) {
    type Output = T;
    type PhantomOutput = Used;
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

impl<T, D: TupDefault> CanCombine<D, Used> for (T, T) {
    type Output = T;
    type PhantomOutput = Used;
    fn combine(self) -> T {
        self.1
    }
}

impl<T, D: TupDefault> CanCombine<Unused, D> for ((), T) {
    type Output = T;
    type PhantomOutput = D;
    fn combine(self) -> T {
        self.1
    }
}

impl<T, D: TupDefault> CanCombine<D, Unused> for (T, ()) {
    type Output = T;
    type PhantomOutput = D;
    fn combine(self) -> T {
        self.0
    }
}
