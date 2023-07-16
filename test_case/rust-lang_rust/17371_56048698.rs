 rust
trait T {}
struct S;
impl T for S {}

fn main() {
  let s : &T = &{S as T};
}
