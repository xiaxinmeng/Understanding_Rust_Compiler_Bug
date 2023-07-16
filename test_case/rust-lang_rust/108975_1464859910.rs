rust
if mixed_codebase.contains(["C", "Rust"]) {
    assert_eq!(mixed_codebase.security, Security::Unsound);
}
