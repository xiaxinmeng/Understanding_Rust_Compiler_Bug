
use std::any::Any;
fn foo(x: &mut dyn Any) {
  println!("{:?}", x.type_id());
}
