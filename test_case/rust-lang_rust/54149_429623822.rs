rust
trait MyTrait1 {
    type X;
    fn get_val(&self) -> Self::X;
}

trait MyTrait2: MyTrait1 where <Self as MyTrait1>::X: Into<u32> {}

struct MyStruct(u32);

impl MyTrait1 for MyStruct {
    type X = u32;
    fn get_val(&self) -> Self::X { self.0 }
}
impl MyTrait2 for MyStruct {}


fn myfunc<T>(v: &T)
where T: MyTrait2,
      <T as MyTrait1>::X: Into<u32>,
{
    println!("{}", Into::<u32>::into(v.get_val()));
}

fn main() {
    let s = MyStruct(10);
    myfunc(&s);
}
