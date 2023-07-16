rust
struct S { // Outer
    field: u8,
}

macro_rules! def_inner_s {
    () => {
        use std::clone::Clone;
        #[derive(Clone)]
        struct S {} // Inner
    };
}

macro_rules! def_test {
    () => {
        use super::*;
        fn test() {
            S {}; // works on stable, reports a missing field on beta
        }
    };
}

mod outer {
    use super::*;

    def_inner_s!();

    mod inner {
        def_test!();
    }
}

fn main() {}
