 rust
fn foo(_: ~int) {}  

fn main() {         
    let a = ~2;     
    do spawn {      
        loop {      
            foo(a); 
        }           
    }               
}                   
