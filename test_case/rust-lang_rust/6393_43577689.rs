
struct MyThing<'r> {
  int_ref: &'r int,
  val: int
}

impl<'r> MyThing<'r> {
  fn new(int_ref: &'r int, val: int) -> MyThing<'r> {
    MyThing {
      int_ref: int_ref,
      val: val
    }
  }

  fn set_val(&'r mut self, val: int) {
    self.val = val;
  }
}


fn main() {
  let to_ref = 10;
  let mut thing = MyThing::new(&to_ref, 30);
  thing.set_val(50);

  println!("{}", thing.val);
}
