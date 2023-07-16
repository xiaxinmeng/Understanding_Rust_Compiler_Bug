rust
fn get_two_mut<T, const N: usize>(this: &mut [T; N], i: usize, j: usize) -> (&mut T, &mut T) {
    let (i, j) = if i < j { (i,j) } else { (j,i) };
    assert!(i < j && j < N);
    
    let s0 = i;
    // The position of the second split relative to the first.
    let s1 = j - i;
    
    let (_, split) = this.split_at_mut(s0);
    let (first, second) = split.split_at_mut(s1);
    (&mut first[0], &mut second[0])
}
