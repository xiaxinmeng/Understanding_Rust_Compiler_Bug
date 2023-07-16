rust
#![feature(concat_idents)]
fn main() {
    let concat_idents!(fn) = 3; // ERROR non-pattern macro in pattern position
    let x = concat_idents!(fn);
}
