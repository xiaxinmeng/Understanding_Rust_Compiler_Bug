 rust
trait HasLifetime<'a>: 'a {
  fn read(self: Box<Self>) -> &'a mut i32;
}

// this is dangerous
fn coerce<'a>(x: Box<HasLifetime<'a>>) -> Box<HasLifetime<'static>> { x }
