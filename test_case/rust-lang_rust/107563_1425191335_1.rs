rust
pub mod a {
    pub fn foo() {}
}

pub mod b {
    pub fn foo() {}
}

pub use a::foo;
pub use b::foo;
