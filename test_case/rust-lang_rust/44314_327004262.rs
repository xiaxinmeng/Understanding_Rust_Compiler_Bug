
let mut v2 = [1, 0, 3, 0, 5, 6];
{
    let (left, right) = v2.split_at_mut(2);
    left[1] = 2;
    right[1] = 4;
}
assert!(v2 == [1, 2, 3, 4, 5, 6]);
