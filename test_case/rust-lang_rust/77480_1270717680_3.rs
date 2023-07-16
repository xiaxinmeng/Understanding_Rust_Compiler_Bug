rust
//fallible method
fn get(&self, index: usize) -> Option<&T>
{
   if self.len() < usize{
       None
   } else {
   Some(&self[index])
   }
}

//panicking method
fn get(&self, index: usize)! -> &T
{
   &self[index]
}

unsafe fn get... something more advanced
}
