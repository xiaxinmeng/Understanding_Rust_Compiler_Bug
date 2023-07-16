 rust
#[test]
fn test_read_from_path() {
    let oldpath = getenv("PATH");
    setenv("PATH", "foo:bar:baz");
    let result = path_env();
    match oldpath {
        Some(p) => setenv("PATH", p.as_slice()),
        None => unsetenv("PATH")
    }
    assert_eq!(result, vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]);
}
