
fn foo(x: i32, y: i32) -> i32 {
  x + y
}

fn bar() -> i32 {
  let f = foo;
  f(1, 2)
}

fn main() {
  println!("{}", bar());
}
