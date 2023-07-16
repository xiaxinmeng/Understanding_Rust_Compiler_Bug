rust
fn funny(v: Vec<i32>) -> Vec<i32> {
    v.iter()
        .scan(0, |acc, x| { *acc += x; Some(*acc) })
        .scan(1, |acc, x| { *acc *= x; Some(*acc) })
        .collect()
}
