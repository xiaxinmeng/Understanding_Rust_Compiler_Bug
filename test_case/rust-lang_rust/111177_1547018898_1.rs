rs
fn make_recursive_type() -> impl Sized {
    [make_recursive_type(), make_recursive_type()]
}
