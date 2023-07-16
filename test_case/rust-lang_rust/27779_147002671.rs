
const MAX_SIZE: usize = 128;

struct StaticBox<T> {
  buffer: [u8; MAX_SIZE]
}

impl StaticBox<T> {
  fn as_ref(&self) -> &T {
    // We know an instance of T has been allocated in self.buffer, but
    // how do we get it out?
    unimplemented!();
  }
}

trait T {
  fn new() -> T;
  fn do_something(&self);
}

struct A { a: u8 }
impl T for A {
  fn new() { A { a: 1 } }
  fn do_something() { unimplemented!() }
}

struct B { b: [u64; 8] }
impl T for B {
  fn new() { B { b: [0; 8] } }
  fn do_something() { unimplemented!() }
}

fn create_and_return_a_T() -> StaticBox<T> {
  let x: StaticBox<T> = unimplemented!(); // initialize an instance of `A` inside |x|.
  x
}

fn main() {
  let a = create_and_return_a_T();
  let b: &T = a.as_ref();
}
