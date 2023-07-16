 Rust
#![feature(optin_builtin_traits)]

trait Marker {}
impl Marker for .. {}
impl !Marker for Bar {}

struct Bar;
impl<T: Marker> From<T> for Bar { fn from(_: T) -> Self { Bar } }
