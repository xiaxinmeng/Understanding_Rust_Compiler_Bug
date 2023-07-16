rust
fn get(&mut self) -> Option<u8> {                   
  if self.head == self.tail { return None; }       
  self.tail = self.tail.wrapping_add(1);           
  Some(self.data[self.tail.wrapping_sub(1) & MASK])
}                                                  
