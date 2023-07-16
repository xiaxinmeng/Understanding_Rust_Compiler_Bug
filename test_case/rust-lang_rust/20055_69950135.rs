
pub fn main() {
  print0("Hello World 1\n\0");
  let _: Box<[i8]> = match true {
    true => { box_1() } // <-- this line is the main difference
    _ => box_2(),
  };
  print0("Hello World 2\n\0");
}
