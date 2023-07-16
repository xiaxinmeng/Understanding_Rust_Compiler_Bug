rust
pub fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let t = *x;
    *x = *y;
    *y = t;
}
