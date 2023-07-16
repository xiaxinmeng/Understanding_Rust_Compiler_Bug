rust
pub fn four(i: usize, b: &mut [i32]) {
    if i >= b.len() {
        return;
    }
    for x in &mut b[.. i].iter_mut() {
        *x += 1;
    }
}
