rust
pub fn push_and_get_ref(v: &mut Vec<i32>) -> &i32 {
    v.push(1);
    v.last().unwrap()
}
