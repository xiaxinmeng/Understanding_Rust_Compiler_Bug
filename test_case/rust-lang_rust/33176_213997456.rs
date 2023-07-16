 rust
type DropperCtxt = Context<Dropper>;
struct Dropper(u32);
impl Drop for Dropper {
  fn drop(&mut self) {
    println!("{}", self.0);
  }
}

fn main() {
  let ctxt = DropperCtxt::new();
  ctxt.push(Dropper(0));
  ctxt.push(Dropper(1));
  ctxt.push(Dropper(2));
  for _ in ctxt.vec() {
  }
}
