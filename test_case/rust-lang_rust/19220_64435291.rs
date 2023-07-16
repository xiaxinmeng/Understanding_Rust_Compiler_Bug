 rust
fn main() {                         
    let rc = RefCell::new(1u);

    let b: &uint = {                
        let a = rc.borrow();        
        *a                          
    };                              
    let mut c = rc.borrow_mut();    
    **c = 3;                        
    println!("{}", *b);             
}                                   
