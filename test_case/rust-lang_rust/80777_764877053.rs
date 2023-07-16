rust
#![feature(staged_api)]

pub trait Foo {}

pub struct Bar;

#[stable(feature = "foobar", since = "1.0.0")]
impl Foo for Bar {}
