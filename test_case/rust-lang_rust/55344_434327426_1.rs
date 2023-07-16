rust
#[allow(unreachable_code, unused_mut)]
pub fn sum_nan() {
    return;

    let mut v = 0;
    assert_eq!(v, 0);
    let x = &mut v;
    assert_eq!(*x, 0);
    *x = 1;
    let y = &mut v;
    assert_eq!(*y, 1);
    assert_eq!(*x, 1);
}
