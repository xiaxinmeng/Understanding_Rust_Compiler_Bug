rust
#![feature(const_panic)]

struct PrintName;

impl PrintName {
    const VOID: ! = panic!();
}

fn main() {
    PrintName::VOID;
}
