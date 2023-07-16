 rust
// lib.rs
include!("foo.rs");

// foo.rs
#[cfg(nope)]
mod bar {
    include!("bar.rs");
}

// bar.rs
omg error
