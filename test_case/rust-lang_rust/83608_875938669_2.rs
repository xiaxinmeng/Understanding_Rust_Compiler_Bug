
let v = &mut [1, 2, 3, 4];
{
    let mut v = v;
    std::mem::swap(v.reserve_mut(2)[0], v[0]);
}
assert_eq!(v, &[3, 2, 1, 4]);
