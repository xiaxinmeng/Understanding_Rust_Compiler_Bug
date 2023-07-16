rust
pub mod bad {
    extern "Rust" { fn foo(x: Option<&mut i32>); }
}

extern "Rust" { fn foo(y: *mut i32); }
pub unsafe fn baz(x: *mut i32) { foo(x) }
