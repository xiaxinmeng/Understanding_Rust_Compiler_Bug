rust
#![stable(feature = "unit_test", since="1.0.0")]
#![feature(staged_api, rustc_attrs)]
#![crate_type="lib"]

#[stable(feature = "unit_test", since="1.0.0")]
pub struct S(u8);

#[stable(feature = "unit_test", since="1.0.0")]
#[rustc_promotable]
pub const fn bar() -> S { S(10) }
