rust
fn _foo() {
    let _z = 1;
    let mut _y = 2;
    let x = _y = _z; //~ WARN unused variable: `x`
}
