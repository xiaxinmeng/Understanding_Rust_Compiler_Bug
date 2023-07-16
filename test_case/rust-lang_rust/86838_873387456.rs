rs
#[crate_type = "lib"]
#![feature(staged_api)]
#![stable(feature = "foo", since = "1.0.0")]

#[stable(feature = "foo", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_foo", issue = "none")]
pub fn foo() {}
// ~^ ERROR attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function to be marked `const`

// ... test other cases, like rustc_const_unstable on impl items
