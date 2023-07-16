rust
fn is_borrowed<T>(x: Cow<T>) -> bool {
    if let Cow::Borrowed(_) = x { true }
    else { false }
}
