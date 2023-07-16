 rust
fn foo(self: &A) -> int {           
    if true {                       
        fail!()                     
    } else {                        
        *bar(self.bar)              
    }                               
}                                   

pub fn main() {}                    

fn bar(_: &'r mut int) -> &'r int {   
    fail!()                         
}                                   

struct A {                          
  bar: @mut int,                    
}                                   
