rust
fn foo(val: &mut bool) {
    match val {
        _ => { val; }
    }
    val;
}
