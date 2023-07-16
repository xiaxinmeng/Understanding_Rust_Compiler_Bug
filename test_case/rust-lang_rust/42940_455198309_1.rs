rust
trait Post<'a, B> {
    //     ^^^^ these are the "captured" parameters, per RFC 1951
    type Output: Future + 'a;
}

impl<'a, B> Post for () {
    default type Output = /* the hidden type that compiler infers */;
}
