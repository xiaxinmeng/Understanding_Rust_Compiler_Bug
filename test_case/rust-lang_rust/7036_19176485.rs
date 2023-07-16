 rust
fn main() {
  let foo = |x:int| x*x; // This creates a closure assigned to foo

  let foo = |x,y| {
    foo(x) + foo(y) // Which foo gets called?
  }
}
