rust
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

#[derive_const(PartialEq)]
struct A(u8);

const fn is_eq(left: &A, right: &A) -> bool {
    left == right
}
