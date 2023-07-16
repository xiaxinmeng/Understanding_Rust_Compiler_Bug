 rust
macro_rules! a {                   
    ($mac:ident) => ($mac!());     
    ($mac:ident 1 $($t:tt)*) => (  
        a!($mac $($t)*);           
        a!($mac $($t)*);           
    )                              
}                                  

fn main() {                        
    macro_rules! doit {            
        () => (let _x = |a: u32| a + 4;)
    }                              
    a!(doit 1 1 1 1 1 1 1 1 1 1);  
}                                  
