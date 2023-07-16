rust
extern crate core;
use core::cmp::PartialEq;

#[repr(packed)]
struct Demo(u8,u32);

impl PartialEq for Demo {
    fn eq(&self, rhs: &Demo) -> bool {
         // This crashes with bus error
         // PartialEq::eq(&self.0, &rhs.0) && PartialEq::eq(&self.1, &rhs.1)
         // This doesn't
         self.0 == rhs.0 && self.1 == rhs.1
    }
}

fn main() {
    let a = Demo(0,4);
    let b = Demo(0,4);                                                                                                                                                      
    if a != b {                                                                                                                                                             
        println!("Hello World!");                                                                                                                                           
    }
}
