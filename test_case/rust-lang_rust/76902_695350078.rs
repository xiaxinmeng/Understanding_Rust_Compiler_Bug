rust
fn myswap<T>(x: &mut[T], i: usize, j: usize) {
    if i == j { return; }
    let (i, j) = (i.min(j), i.max(j));
    let (a, b) = x.split_at_mut(j);
    std::mem::swap(&mut a[i], &mut b[0]);
}
