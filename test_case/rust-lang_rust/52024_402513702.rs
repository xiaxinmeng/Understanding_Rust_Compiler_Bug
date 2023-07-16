rust
existential type Cmp<T>: std::cmp::PartialEq<T>;
//~^ ERROR could not find defining uses

// not a defining use, because it doesn't define *all* possible generics
fn cmp() -> Cmp<u32> {
    5u32 //~ mismatched types
}

existential type Cmp2<T>: std::cmp::PartialEq<T>;
//~^ ERROR could not find defining uses

// not a defining use, because it doesn't define *all* possible generics
// it only defines `Cmp2` for `T: SomeBounds`
fn cmp2<T: std::cmp::PartialEq<T>>(t: T) -> Cmp2<T> {
    t
}
