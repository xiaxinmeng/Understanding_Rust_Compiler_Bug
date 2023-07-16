rust
impl<T> Vec<T>{
fn get(&self, index: usize) -> Option<&T>
{
   if self.len() < usize{
       None
   } else {
   Some(&self[index])
   }
}
}
