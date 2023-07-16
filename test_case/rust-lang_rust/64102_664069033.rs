rust
fn push_cap(v: &mut Vec<i32>) {
    for i in 0..4 {
        assume_used(v.as_ptr());
        v.push(bench_black_box(i));
        assume_used(v.as_ptr());
    }
}
