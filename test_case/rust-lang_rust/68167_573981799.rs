rust
pub fn set_to_zero(v: &mut Vec<i32>) {
    let base = v.as_mut_ptr();
    for i in 0..v.len() {
        unsafe { *base.add(i) = 0 };
        //v[i] = 0;
    }
}
