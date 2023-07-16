 rust
trait Movable: Positioned {
  fn translate(dx: int, dx: int) {
    let s = self as Positioned;
    s.SetX(s.X() + dx);
    s.SetY(s.Y() + dy);
  }
}
