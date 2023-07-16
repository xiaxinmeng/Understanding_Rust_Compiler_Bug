rust
fn main() {
    // nothing was specified, so use the default
    let _ = foo::<String>(Some("I'm a string")); // ERROR: &str vs String

    // nothing was specified, so use the default
    let _ = foo::<String>(None);
}
