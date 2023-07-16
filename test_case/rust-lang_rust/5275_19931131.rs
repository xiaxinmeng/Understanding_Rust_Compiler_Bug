 rust
fn foo(self_: &A) -> int {           
    if true {                       
        fail!()                     
    } else {                        
        *bar(self_.bar)              
    }                               
}                                   

pub fn main() {}                    

fn bar<'r>(_: &'r mut int) -> &'r int {   
    fail!()                         
}                                   

struct A {                          
  bar: @mut int,                    
}                                   
