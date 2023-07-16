rust
pub trait EnvFuture {
    type Item;

    fn boxed_result<'a>(self) where Self: Sized, Self::Item: 'a, {
    }
}

struct Foo;

impl<'a> EnvFuture for &'a Foo {
    type Item = ();
}
