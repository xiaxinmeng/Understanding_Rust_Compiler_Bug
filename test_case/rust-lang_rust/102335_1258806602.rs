rust
#![feature(associated_const_equality)]
#![crate_type = "lib"]

trait T {
    type A: S<C<X = 0i32> = 34>;
}

trait S {
    const C: i32;
}
