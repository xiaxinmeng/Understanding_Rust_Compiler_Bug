rust
trait Foo { type Out; }
impl Foo for () { type Out = &'static u32; }

fn main() {
  let a = 22;
  let b: <() as Foo>::Out = &a;
}
