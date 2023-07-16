rust
fn main() {
    let x = &"hello";
    let mut y = 0 as *const _;
    y = x as *const _;
              ^^^^^^^^ cannot cast to a pointer of an unknown kind.
}
