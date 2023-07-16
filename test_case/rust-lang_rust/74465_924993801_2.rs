rust
   // maybe poorly named, just an example
   pub struct AlreadySet<'a, T> { 
       pub already_set_to: &'a T,
       pub value: T,
   }
   pub fn set(&self, value: T) -> Result<&T, AlreadySet<'_, T>> {}
   