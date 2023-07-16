 rust
fn lines() -> Vec<String> {
    let mut stdin = std::old_io::stdio::stdin();
    let mut locked = stdin.lock();
    locked.lines().map( |line| {
        line.unwrap().trim().to_string()
    }).collect()
}
