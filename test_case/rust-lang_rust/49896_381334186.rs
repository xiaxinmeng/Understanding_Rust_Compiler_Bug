rust
#![crate_type = "rlib"]
#![no_std]
pub fn foo() {
    assert!(::core::prelude::v1::CharExt::is_digit('1', 10));
    assert!(::core::prelude::v1::StrExt::is_empty(""));
    assert!(!::core::num::Float::is_nan(1.0));
}
