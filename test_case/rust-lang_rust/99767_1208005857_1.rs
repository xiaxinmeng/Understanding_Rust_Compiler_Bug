rust
fn bar() {
    // check that some invariants are upheld before doing some unsafe
    if !invariant_upheld { return; }

    let x: Foo = || unsafe { unsf(); };
}
