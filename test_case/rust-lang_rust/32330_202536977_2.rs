 rust
fn foo<'a>(x: &'a u32, y: &'a u32) {
    bar(x, y) // OK: 'b and 'c can both be bound to `'a` (or, in fact, the call site)
}

fn bar<'b, 'c>(x: &'b u32, y: &'c u32) {
    foo(x, y) // OK: 'a is inferred to the call site
}
