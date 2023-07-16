


mod sub1 {                                                                         
    fn foo() {} // implementation 1                                                
}                                                                                  

mod sub2 {                                                                         
    fn foo() {} // implementation 2                                                
}                                                                                  

use sub1::foo; //~ note first imported here                                        
use sub2::foo; //~ error a value named `foo` has already been imported in this module [e0252]

fn main() {}
