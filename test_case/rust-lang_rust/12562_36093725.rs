 rust
struct A: B { ... }

fn foo(_: &B) {}

fn bar(a: &A) { foo(a) }
