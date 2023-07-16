
fn foo(f: option<fn&()>) { f.iter(|x|x()) }
fn bar() {}
fn main() { foo(some(bar)); }
