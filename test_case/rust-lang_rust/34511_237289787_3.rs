 rust
trait FooReturn<'a,'b> {
    type Type: Iterator<Item=u32>;
}

impl<'a,'b> FooReturn<'a,'b> for () {
    default type Type = XXX; // can't really be specialized, but wev
}
