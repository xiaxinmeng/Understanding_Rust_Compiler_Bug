rust
mod foo {
    pub use bar::*;
    pub mod bar {
        pub trait Foo {
            fn foo();
        }
    }
}

pub use foo::bar::*;
pub use foo::*;
