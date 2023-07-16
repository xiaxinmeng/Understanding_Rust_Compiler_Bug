
class r {
  let mut x: int;
  new() {
    self.x = 1;
  }
  drop {
    self.x += 1;
    io::println("self.x: " + int::str(self.x));
  }
}

fn main() {
  let mut res = r();

  let mut _v = ~[mut];
  _v <- ~[mut res] + _v;
}
