rust
fn is<T>(&self) -> bool where T: Any;
fn downcast_ref<T>(&self) -> Option<&T> where T: Any;
fn downcast_mut<T>(&mut self) -> Option<&mut T> where T: Any;
fn downcast<T>(Box<self>) -> Result<Box<T>, Box<Any>> where T: Any ;
