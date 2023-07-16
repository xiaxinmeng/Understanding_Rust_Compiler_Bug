rust
mod name {
    // Finds `name_methods.rs` automatically if it's in conventional location,
    // or you can use #[path = "path/to/name_methods.rs"] explicitly, this is
    // described in the book https://doc.rust-lang.org/nightly/book/second-edition/ch07-01-mod-and-the-filesystem.html
    mod name_methods;
}
