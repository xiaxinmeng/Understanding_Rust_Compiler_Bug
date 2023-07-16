
extern "C" {
  fn malloc(len: usize) -> *mut u8;
}

fn main() {
  println!("Allocated at {:?}",unsafe{malloc(100)});
}
