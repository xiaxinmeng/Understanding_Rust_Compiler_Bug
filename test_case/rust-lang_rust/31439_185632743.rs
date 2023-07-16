 rust
fn lines() -> Vec<String> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let result = handle.lines().map(|line| line.unwrap()).collect();
    result
}
