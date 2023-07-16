rust
#![cfg_attr(feature = "unstable", feature(inline_const))]

#[cfg(feature = "unstable")]
pub fn foo() {
    const { 1 + 1 };
}
