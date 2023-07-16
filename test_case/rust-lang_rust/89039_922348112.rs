Rust
~/test_rustdoc_json$ tail -n+1 Cargo.toml foo/Cargo.toml foo/src/lib.rs bar/Cargo.toml bar/src/lib.rs 
==> Cargo.toml <==
[workspace]

members = [
    "foo",
    "bar",
]

==> foo/Cargo.toml <==
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]

==> foo/src/lib.rs <==
pub struct Foo;

==> bar/Cargo.toml <==
[package]
name = "bar"
version = "0.1.0"
edition = "2021"

[dependencies]
foo = { path = "../foo" }

==> bar/src/lib.rs <==
pub mod bar {
    pub use foo::Foo;
}

/// Bar
pub use bar::Foo;
