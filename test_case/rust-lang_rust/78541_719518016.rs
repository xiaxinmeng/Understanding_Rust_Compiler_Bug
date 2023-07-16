rust
pub fn get_two_mut_safe(this: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i < this.len() && j < this.len() && i != j);
    if i > j {
        let (first, second) = this.split_at_mut(i);
        (&mut second[0], &mut first[j])
    } else {
        let (first, second) = this.split_at_mut(j);
        (&mut first[i], &mut second[0])
    }
}
