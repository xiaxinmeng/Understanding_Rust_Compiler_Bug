rust
pub fn test() {
    Vec::<()>::new()
        .into_iter()
        .filter(|_| true)
        .collect::<Vec<_>>();
}
