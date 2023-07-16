 rust
fn main() {
    let mut x = 4;
    let y = &mut x as *mut i32;
    x = 5;
    unsafe ( assert_eq!(5, *y) };
}
