rust
pub fn foo() {
    let x;
    if true && {x = 4; true} && x == 4 {}
}
