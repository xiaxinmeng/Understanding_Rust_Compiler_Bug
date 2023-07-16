rust
mod serde;
mod foo {
    // In Rust 2015, this comes from the module at crate root.
    //
    // In Rust 2018, this comes from the crate `serde` (presuming there is one), or otherwise
    // is an error.
    use serde::bar;
}
