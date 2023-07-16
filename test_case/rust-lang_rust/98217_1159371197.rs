rust
#![deny(unconditional_recursion)]

pub fn a() {
    b()
}

pub fn b() {
    a()
}
