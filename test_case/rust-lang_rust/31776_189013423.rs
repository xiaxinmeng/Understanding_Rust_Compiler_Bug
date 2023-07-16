 rust
pub trait Tr {
    type A;
}
pub struct S;

fn f() {
    struct Z;

    impl ::Tr for ::S {
        type A = Z; // Private-in-public error unless `struct Z` is pub
    }
}

fn main() {}
