rust
fn main() {
    // infer the type *if possible*...
    let _ = foo::<_>(Some("I'm a string"));

    // ...otherwise, use a default
    let _ = foo::<String>(None);
}
