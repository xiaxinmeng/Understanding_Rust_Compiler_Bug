
let mut v = &mut [1, 2, 3, 4];
{
    let a = v.reserve_mut(2);
    let b = v.reserve_mut(2);
    for (a,b) a.iter_mut().zip(b.iter_mut()) { 
        std::mem::swap(a, b);
    }
}
assert_eq!(v, &[3, 4, 1, 2]);
