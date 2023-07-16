 rust
struct A;                     

impl Drop for A {             
    fn drop(&mut self) {      
        println!("hello");    
    }                         
}                             

fn foo() -> A { A }           

fn main() {                   
    let a = &(foo() as A);    
    println!("here: {:p}", a);
}                             
