 rust
pub mod bar {
    pub fn f() {}
}

mod foo {
    pub use super::bar;
}

fn main() {
    foo::bar::f();
    bar::f();
}
