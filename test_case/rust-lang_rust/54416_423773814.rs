
fn foo(x: i32, y: i32) -> i32 {
  x + y
}

fn main() {
  let f = foo;
  println!("{}", f(1, 2));
}
