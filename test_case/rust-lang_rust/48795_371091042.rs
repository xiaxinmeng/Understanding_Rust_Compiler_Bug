rust
use std::ptr;
fn main() {
    let array = [1, 2, 3, 4];
    let slice1 = &array[0..1];
    let slice2 = &array[0..3];
    println!("{:p} == {:p} ? {}", slice1, slice2, ptr::eq(slice1, slice2));
}
