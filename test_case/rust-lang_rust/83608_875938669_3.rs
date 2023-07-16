
let v = &mut [1, 2, 3, 4];
{
    let mut v = v;
    let a = v.reserve_mut(1);
    v.reserve_mut(1);
    let b = v.reserve_mut(1);
    std::mem::swap(a, b);
}
assert_eq!(v, &[3, 2, 1, 4]);
