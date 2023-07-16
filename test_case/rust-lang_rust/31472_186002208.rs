 rust
fn foo() -> i32 { panic!(); } // OK
fn foo() -> i32 { panic!(); () } // expected `i32`, found `()` - http://is.gd/qVOTyf
