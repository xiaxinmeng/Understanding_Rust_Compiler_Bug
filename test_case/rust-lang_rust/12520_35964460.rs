
fn total_order(af: f32, bf: f32) -> Ordering {
    let a: i32 = unsafe {transmute(af)};
    let b: i32 = unsafe {transmute(bf)};
    if (a & b) >= 0 { // both positive, or different signs
        a.cmp(b)
    } else { // both negative
        b.cmp(a)
    }
}
