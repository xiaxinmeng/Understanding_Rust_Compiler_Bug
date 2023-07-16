rust
pub fn foo(v: &Option<Vec<i32>>) {
    match v {
        Some([]) => {}
        _ => {}
    }
}
