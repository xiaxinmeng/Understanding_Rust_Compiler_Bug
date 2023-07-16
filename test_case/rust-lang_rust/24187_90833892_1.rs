 rust
fn foo(v: &mut Broken) {
    v.x = 42;
    v.do_stuff(); // we don't use the return value here
    // now where is the chance to destruct 42?
}
