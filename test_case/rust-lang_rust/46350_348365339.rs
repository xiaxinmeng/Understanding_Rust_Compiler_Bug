rust
fn closure_to_loc() {
    let mut x = |c| c + 1;
    x = |c| c + 1;
}
