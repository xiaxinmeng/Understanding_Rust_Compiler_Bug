 Rust
fn with_temp_dir<T>(base: &Path, suffix: &str, f: |temp_dir: &Path| -> T) -> T {
    let prev_path = os::getcwd();
    let temp_dir = TempDir::new_in(base, suffix);
    os::change_dir(temp_dir.path());
    let result = f(temp_dir.path());
    os::change_dir(prev_path);
    fs::rmdir_recursive(temp_dir);
    result
}
