 rust

struct MyStruct(Box<int>);

impl Drop for MyStruct {
  fn drop(&mut self) {
    let &MyStruct(ref n) = self;
    println!("dropping {}", n);
  }
}

fn foo() -> Result<(MyStruct, MyStruct), MyStruct> {
  Ok((MyStruct(box 0), try!(Err(MyStruct(box 1)))))
}

fn main() {
  foo();
}
