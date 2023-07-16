
enum O {
    A(i32),
    B(i32)
}

fn main() {
  let (O::A(x) | O::B(x)) = O::B(1);
  println!("{}", x)
}
