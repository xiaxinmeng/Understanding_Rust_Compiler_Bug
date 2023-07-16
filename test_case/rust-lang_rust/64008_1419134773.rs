rust
fn main() {
    let get = |path: &str| || {
        assert!(path.starts_with('/'));
    };
}
