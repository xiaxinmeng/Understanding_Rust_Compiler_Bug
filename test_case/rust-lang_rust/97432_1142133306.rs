rust
pub use foo::Trait;
pub use foo::Trait as Reexport;

mod foo {
    pub trait Trait {}
}
