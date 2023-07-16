 rust
macro_rules! gen_test {
  () => (
    fn test() {
      box 1;
    }
  );
}

gen_test!();

fn main() {
  test();
  //box 1;  // error, requires #![feature(box_syntax)]
}
