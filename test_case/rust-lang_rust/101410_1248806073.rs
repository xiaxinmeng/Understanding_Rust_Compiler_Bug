rust
#![feature(let_else)]
fn main() {
    let x: Option<u8> = Some(1);
    let Some(y) = x else {
        panic!();
    };
}
