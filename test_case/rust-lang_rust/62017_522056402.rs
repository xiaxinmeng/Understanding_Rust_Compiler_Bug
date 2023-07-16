
extern crate a;
  
pub fn takep(arg: fn() -> ()) -> () {}  

pub fn example1() {
  takep(a::voidfn);
} 
