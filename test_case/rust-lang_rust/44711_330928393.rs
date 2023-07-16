rust
fn with_p4branch<R, F: FnOnce(&str) -> R>(branch: &git2::Branch, f: F) -> Result<R, SomeErrorIDontRemember>;

// Here's how it was used:
fn get_p4branch(branch: &git2::Branch) -> Result<String, SomeErrorIDontRemember> {
    with_p4branch(branch, String::from)
}
