 rust
mod foo {
    mod bar { pub struct Bar; }
    pub use self::bar::Bar;
}
pub use foo::*; // This defines Bar in both namespaces
pub fn Bar() {} // This shadows the old definition in the value namespace

fn main() {
    Bar; // This refers to the shadowing function
    fn g(_b: Bar) {} // This refers to the struct, since it was not shadowed in the type namespace
}
