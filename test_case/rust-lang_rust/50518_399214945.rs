
use std::marker::PhantomData;

struct Foo<T>(PhantomData<T>);

impl<T> Foo<T>{
  const OP: fn(&T) = |_t| {};
  
  fn op(_t: &T){}
  
  fn foo(&mut self, t: T){
    //does not work
    Self::OP(&t);
    //does work
    Self::op(&t);
  }
}

fn main(){
  Foo(PhantomData).foo(1);
}
