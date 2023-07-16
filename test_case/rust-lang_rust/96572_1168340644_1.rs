rust
fn foo(b: bool) -> Option<impl Copy> {
    if b {
        return None;
    }
    match foo(!b) {
        Some((mut x, mut y)) => {
            x = 42;
            y = "foo";
        }
        None => {}
    }
    None
}
