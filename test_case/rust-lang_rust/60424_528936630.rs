rust
async fn func(s: &str) -> &[&str] {
    &[s] as &[&str]
}
