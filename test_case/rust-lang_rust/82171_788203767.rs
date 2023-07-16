rust
fn foo<T>(a: T) -> impl Iterator<Item = String> + 'static
where
    T: std::fmt::Display,
{
    std::iter::once(a.to_string())
}

// What are those?
trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

fn bar1<'a>(a: &'a str) -> impl Iterator<Item = String> + 'static + Captures<'a> {
    foo(a)
}
