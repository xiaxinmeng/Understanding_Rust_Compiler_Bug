rust
fn main() {
    let x = &"hello";
    let mut y = x as *const _;
    y = 0 as *const _;
}
