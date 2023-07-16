rust
pub fn fold_ref(s: &[i32]) -> Option<&i32> {
    let mut r = None;
    s.iter().for_each(|i| r = Some(i));
    r
}
