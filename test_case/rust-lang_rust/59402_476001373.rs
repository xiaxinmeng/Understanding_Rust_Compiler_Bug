rust
trait X<'x>: Sized {}

impl<U> X<'_> for U {}

fn hide<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl X<'b> + 'a {
    x
}
