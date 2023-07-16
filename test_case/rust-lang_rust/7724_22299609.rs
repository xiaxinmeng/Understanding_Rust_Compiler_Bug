
use std::libc::c_void;

pub type Foo = c_void;

#[deriving(Clone)]
struct Bar {
  priv foo: *Foo,
}
