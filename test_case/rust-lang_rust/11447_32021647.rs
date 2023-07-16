 rust
use std::libc::{malloc, size_t};

unsafe fn foo() { malloc(10u as size_t); }
