rust
#[allow(unreachable_code)]
pub fn sum_nan() {
    return;

    let v = 0;
    assert_eq!(v, 0);
    std::mem::drop(v);
    std::mem::drop(v);
    assert_eq!(v, 1);
}
