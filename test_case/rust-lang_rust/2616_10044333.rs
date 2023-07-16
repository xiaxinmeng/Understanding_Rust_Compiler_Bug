 rust
struct Point {
  mut x: int,
  mut y: int
}

trait Positioned {
  fn X()-> int;
  fn Y()-> int;
  fn SetX(int);
  fn SetY(int);
}

trait Movable: Positioned {
  fn translate(dx: int, dy: int) {
    self.SetX(self.X() + dx);
    self.SetY(self.Y() + dy);
  }
}

struct Entity {
  mut pos: Point,
}

impl Entity: Positioned {
  fn X()-> int { self.pos.x }
  fn Y()-> int { self.pos.y }
  fn SetX(v: int) { self.pos.x = v; }
  fn SetY(v: int) { self.pos.y = v; }
}

impl Entity: Movable { }

fn main() {
  let mut e: Entity = Entity {
    pos: Point {x: 0, y: 0},
  };

  io::println(fmt!("%?, %?", e.X(), e.Y()));
  e.translate(5, 10);
  io::println(fmt!("%?, %?", e.X(), e.Y()));
}
