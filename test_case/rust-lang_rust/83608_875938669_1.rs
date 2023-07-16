
let v = &mut [1, 2, 3, 4];
let [a, b] = v.get_many_mut([0, 2]).unwrap();
std::mem::swap(a, b);
assert_eq!(v, &[3, 2, 1, 4]);
