 rust
trait SelectPort<T> {
    fn recv_ready(self) -> Option<T>;
}

impl <T> SelectPort<T> for PortOne<T> {
  ...
}
impl <'self, T> SelectPort<T> for &'self mut Port<T> {
  ...
}
