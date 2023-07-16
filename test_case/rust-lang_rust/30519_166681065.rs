 rust
fn main() {                      
    let foo = &mut true;         
    (|| {                        
        let bar = &mut *foo;     
        (move |b: bool| *bar = b)
    });                          
}                                
