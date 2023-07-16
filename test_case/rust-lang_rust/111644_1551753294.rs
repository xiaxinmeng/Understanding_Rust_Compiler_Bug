rust
#![feature(let_chains)]
pub fn foo() {
    let x;
    if let _t = 0 && true && {x = 4; true} && x == 4 {}
}
