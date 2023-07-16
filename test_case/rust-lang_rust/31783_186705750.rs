 rust
mod foo {
    mod bar { pub fn f() {} }
    pub fn bar() {}
}

// This effectively imports `foo` in the value namespace. I say "effectively" because
// trying to use the imported name `foo` in the type namespace is a privacy error,
// not an unresolved name error.
use foo::bar;

fn main() {
    bar();
    // bar::f(); // If this line were uncommented, the import of `foo` would be a privacy error.
}
