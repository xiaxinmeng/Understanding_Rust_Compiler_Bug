
fn foo(f: fn&()) { f() }
fn bar() {}
fn main() { foo(||bar()); }
