rust
#![feature(existential_type)]

use std::fmt::{Debug, Display};

trait TraitB {
    type AssocB;
}

trait TraitC {
}

// Failing case:
existential type Bar: Debug + TraitB<AssocB = impl Send>;
// Working case:
existential type Bar: Debug + TraitB<AssocB = _0>;
existential type _0: Send;

impl TraitB for i32 {
    type AssocB = u32;
}

impl TraitC for i32 {
}

fn a() -> Bar {
    42
}

fn main() {}
