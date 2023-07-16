
fn total_order(af: f32, bf: f32) -> Ordering {
    let a: i32 = unsafe {transmute(af)};
    let b: i32 = unsafe {transmute(bf)};
    let mask: i32 = (a & b) >> 31; // if both negative, all ones else all zeroes
    (a ^ mask).cmp(b ^ mask);
}
