rust
fn truncate_with_old_drop_order<T>(vec: &mut Vec<T>, len: usize) {
    while vec.len() > len {
        vec.pop();
    }
}
