 rustc
...
fn make_make_a() -> A {
    let b: ~B = ~B {i:1};
    let bb: & B = b;  // error: `*b` does not live long enough
    make_a(bb)
}
...
