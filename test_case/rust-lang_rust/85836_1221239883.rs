rust
fn foo(x: &mut String) {
    let r = &*x;
    x.push_str("asdf");
    return; // comment this to get a borrowck error
    dbg!(r);
}
