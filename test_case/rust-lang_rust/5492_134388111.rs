
#[repr(union)]
pub struct XEvent {
  pub type_: c_int,
  pub xany: XAnyEvent,
  // ...
  pub pad: [c_long; 24],
}
