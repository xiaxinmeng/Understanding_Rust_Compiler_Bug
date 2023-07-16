rust
// run-pass

fn or_chain() {
    let mut z = 0;
    if false || { z = 3; false} || z == 3 {}
    assert_eq!(z, 3);
}

fn main() {
    or_chain()
}
