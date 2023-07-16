rust
fn nonempty_env(name: &str) -> Option<String> {
    match env::var(name) {
        Ok("") | Err(env::VarError::NotPresent) => None,
        Err(e) => panic!("error looking up {}: {}", name, e),
        Ok(s) => Some(s),
    }
}
