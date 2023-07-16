 rust
foo(s.as_ref())
// vs
foo(s.as_bytes().as_ptr())
