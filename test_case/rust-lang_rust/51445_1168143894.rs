rust
impl<'a> GatLike<'a, String> for Foo {
    type Ref = &'a String;
}

impl<'a> GatLike<'a, <String as Deref>::Target> for Foo {
    type Ref = &'a <String as Deref>::Target;
}
