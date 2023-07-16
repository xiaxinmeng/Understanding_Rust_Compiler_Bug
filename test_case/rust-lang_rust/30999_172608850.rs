
struct Context;  

trait ctx{
  fn f1(&self) -> String;  
  fn f2(&self) ;
}

impl ctx for Context{ 
  fn f1(&self) -> String{
      abc
  }
}   
