rust
fn main() {
    let x = (1, 2, 3);
    match x {
        (_a, 1, 1) => {}
        (_a, ..) => {}
    }
}
