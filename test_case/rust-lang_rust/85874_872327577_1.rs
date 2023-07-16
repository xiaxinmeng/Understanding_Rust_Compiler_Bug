rust
pub fn vec_iterator_cast(vec: Vec<isize>) -> Vec<usize> {
    vec.into_iter().map(|e| e as usize).collect()
}
