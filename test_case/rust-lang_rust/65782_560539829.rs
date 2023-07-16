rust
fn evil_mut() {
    let mut a = ();
    let b = &a;
    a = ();
    let c = *b;
}
