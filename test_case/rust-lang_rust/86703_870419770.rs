rust
trait Yokeable<'a> {
    type Output: 'a;
}

fn project(_: for<'a> fn(<() as Yokeable<'a>>::Output)) {}

impl<'a> Yokeable<'a> for () {
    type Output = ();
}

fn main() {
    project(|_| {});
}
