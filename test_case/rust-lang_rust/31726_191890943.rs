 rust
mod foo {
    pub mod bar { pub mod bar { } }
    pub use baz::bar::*;
}

mod baz {
    pub use foo::bar;
}

fn main() { }
