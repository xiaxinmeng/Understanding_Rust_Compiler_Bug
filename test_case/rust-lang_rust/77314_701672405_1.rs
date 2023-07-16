rust
fn main() {
    let a: &[u8] = &[];
    let b: &Vec<u8> = &vec![];
    a == b;
//    ^^ I changed the `>` to `==`
}
