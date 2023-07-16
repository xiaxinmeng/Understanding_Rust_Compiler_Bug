 rust
mod foo {
    pub fn bar() {}
    pub mod bar { pub fn baz() {} }
}

mod qux {
    pub use foo::bar;
}

fn main() {
    foo::bar();
    foo::bar::baz();
    qux::bar();
    qux::bar::baz();
}
