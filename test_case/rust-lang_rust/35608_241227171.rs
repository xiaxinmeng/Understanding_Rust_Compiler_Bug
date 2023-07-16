 Rust
fn main() {
    let mut x = 4;
    let y = &mut x as *mut i32;
    x = 3; // this assignment is not dead
    unsafe ( assert_eq!(5, *y) };
    x = 5; // and neither is this one
    unsafe ( assert_eq!(5, *y) };
}
