rust
// check-pass

#![deny(unused_unsafe)]

unsafe fn foo() {}
unsafe fn bar() { unsafe { foo(); } }
