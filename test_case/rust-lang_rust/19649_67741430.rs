 rust
pub extern { #[deprecated] fn foo(); } // missing stability marker
#[deprecated] pub extern fn foo() {} // has stability marker
