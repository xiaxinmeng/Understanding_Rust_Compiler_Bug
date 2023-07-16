rust
#![feature(type_alias_enum_variants)]

enum E {
    A,
    B
}

type E2 = E;

fn main() {
    use E2::*;
    // ...
}

