rust
pub mod foo {
    pub type X = u8;
}

pub mod bar {
    pub type X = u8;
}

pub use foo::*;
pub use bar::*;
//~^ ERROR the name `X` is defined multiple times

pub fn main() {}
