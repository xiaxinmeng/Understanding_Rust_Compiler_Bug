
pub fn main() {              
  os::args();                
  task::spawn(||());
  fail!();
}                            
