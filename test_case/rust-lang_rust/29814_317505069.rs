rust
pub mod foo {
    pub enum Bar {
        A,
        B,
        C,
    }
}

pub use foo::Bar::A as Apple;

pub fn main() {}
