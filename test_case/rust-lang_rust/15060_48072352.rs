
use break1::MyGuy;

mod break1;

impl MyGuy {
  fn do2() { println!("do 2"); }
}

fn main() {
  MyGuy::do1();
  MyGuy::do2();
}
