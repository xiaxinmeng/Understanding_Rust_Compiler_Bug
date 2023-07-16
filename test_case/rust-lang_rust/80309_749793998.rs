rust
pub fn ptr(x: *const i8) -> *const i8 {
    x.wrapping_offset(-(x as isize)).wrapping_offset(x as isize)
}
pub fn zero(x: *const i8) -> isize {
    (ptr(x) as isize).wrapping_add(-(x as isize))
}
pub fn qux(x: &[i8]) -> i8 {
    x[zero(x.as_ptr()) as usize]
}

fn main() {
    let z = vec![42, 43];
    println!("{}", qux(&z));
}
