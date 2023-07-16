rust
fn get_two_mut<T, const N: usize>(this: &mut [T; N], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i < N && j < N && i != j);
    let ptr = this.as_mut_ptr();
    unsafe { (transmute(ptr.add(i)), transmute(ptr.add(j))) }
}
