 rust
fn foo3(t0: &mut &mut int) { //~ NOTE previous borrow ends here
    let t1 = &mut *t0;
    let p: &int = &**t0;
}
