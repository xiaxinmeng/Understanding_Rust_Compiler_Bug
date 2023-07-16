 rust
pub fn pop_front<M>(mut ml: MoveList<M>) -> (Option<M>, MoveList<M>) {
    match ml.take() {
        None => (None, None),
        Some(box MoveNode {m, n }) => (Some(m), n)
    }
}
