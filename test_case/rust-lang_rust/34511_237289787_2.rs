 rust
trait FooReturn<'a,'b> {
    type Type: Iterator<Item=u32>;
}

impl<'a,'b> FooReturn<'a,'b> for () {
    type Type = XXX;
}
