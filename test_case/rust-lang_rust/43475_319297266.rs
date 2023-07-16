rust

pub trait Param<P> {
    fn param(p: P) -> Self;
}

pub trait Test: Sized {
    type T: Param<Self::Item>;
    type Item;
}

// Compiles
fn test<T: Test>(p: T::Item) -> T::T {
    T::T::param(p)
}

// Does not compile
fn test2<T: Test<Item = u8>>(p: T::Item) -> T::T {
    T::T::param(p)
}

fn main() {
}
