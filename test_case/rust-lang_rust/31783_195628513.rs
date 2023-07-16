 rust
pub fn foo() {}
mod foo { pub fn f() {} }

mod bar {
    pub use foo; // This publicly imports `foo` in the value namespace and privately imports `foo` in the type namespace.
    fn g() { foo::f() }
}

fn main() {
    bar::foo();
}
