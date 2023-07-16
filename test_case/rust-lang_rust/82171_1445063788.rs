rs
trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

fn bar1<'a>(a: &'a str) -> impl Iterator<Item = String> + 'static + Captures<'a> {
    foo(a)
}
