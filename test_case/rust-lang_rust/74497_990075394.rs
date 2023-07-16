rust
pub async fn foo<'a, F, T>(f: F) -> bool
where
    F: Fn(&'a u8) -> T,
    T: Future<Output = bool> + 'a,
