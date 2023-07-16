 rust
trait Lifetime<'a> {
    type Out;
}
impl<'a> Lifetime<'a> for () {
    type Out = &'a ();
}
fn foo<'a>(x: &'a ()) -> <() as Lifetime<'a>>::Out {
    x
}

fn takes_lifetime(f: for<'a> fn(&'a ()) -> <() as Lifetime<'a>>::Out) {
}
fn main() {
    takes_lifetime(foo);
}
