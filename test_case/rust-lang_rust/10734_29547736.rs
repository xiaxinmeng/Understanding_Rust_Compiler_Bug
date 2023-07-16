 rust
struct Foo;                      

impl Drop for Foo {              
    fn drop(&mut self) {         
        println!("dropping");
    }                            
}                                
fn main() {                      
    if true {                    
        let _a = Foo;            
    }                            
}                                
