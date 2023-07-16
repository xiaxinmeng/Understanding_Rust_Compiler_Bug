rust
pub struct PanicInfo<'a> {
    payload: Payload<'a>,
    location: &'a Location<'a>,
}

enum Payload<'a> {
  Any(&'a (dyn Any + Send)),
  Message(&'a fmt::Arguments<'a>),
}
