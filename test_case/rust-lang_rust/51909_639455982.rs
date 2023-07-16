rust
const fn foo(a: &i32, b: &i32) -> bool { unsafe { a.as_ptr() == b.as_ptr() } }
