rust
pub fn push_pop(v: &mut Vec<i32>) -> i32 {
    v.push(1);
    v.pop().unwrap()
}
