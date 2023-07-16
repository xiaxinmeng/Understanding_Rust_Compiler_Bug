
struct Foo<T>(T);

impl<T> Foo<T>{
  const OP: fn(T) = |_t| {};
  
  fn foo(self){
    //does not work
    Self::OP(self.0);
  }
}

fn main(){
  Foo(1).foo();
}
