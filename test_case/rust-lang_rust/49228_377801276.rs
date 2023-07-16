rust
pub fn square(v: &mut Vec<i32>, s: &[i32]) {
    v.extend_from_slice(s);
}

pub fn square2(v: &mut Vec<i32>, s: &[i32]) {
    v.extend(s);
}
