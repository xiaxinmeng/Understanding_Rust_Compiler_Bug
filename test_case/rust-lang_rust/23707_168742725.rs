 rust
trait Foo<T> {
    type Out;
}

impl Foo<()> for () {
    type Out = ();
}

//Removing this impl stops the error, despite being unrelated to the call.
impl<A, B> Foo<(A, B)> for () where (): Foo<A, Out=B> {
    type Out = ();
}

fn bar<T>(_: T) where (): Foo<T> {}

fn main() {
    bar(());
}
