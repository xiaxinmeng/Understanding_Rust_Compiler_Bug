rust
#[non_exhaustive]
pub struct Foo {}

#[repr(transparent)]
pub struct Baz(pub Foo);
