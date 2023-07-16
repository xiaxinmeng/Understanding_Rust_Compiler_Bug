 rust
#[inline(never)]
fn huge_fn(x: &str, d: int) {
  println!("{}: {}", x, d);
}

fn rec(d: int) {
  huge_fn("Test", d);
  rec(d+1);
}

pub fn main() {
  rec(0);
}
