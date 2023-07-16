rs
fn bar3<'a>(a: &'a str) -> impl Iterator<Item = String> + 'a {
    foo(a)
}
