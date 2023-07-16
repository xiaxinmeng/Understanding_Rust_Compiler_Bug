 rust
#![feature(unsafe_destructor)]

struct Test<T:Iterator>(T);

#[unsafe_destructor]
impl <A, T:Iterator<Item=A>> Drop for Test<T> {
  fn drop(&mut self) {}
}

fn main() {
  Test(2);
}
