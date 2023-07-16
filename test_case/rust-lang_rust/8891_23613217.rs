 rust
fn main() {               
    let mut a = ~"test";  
    {                     
        let c = &mut a[2];
        *c = 0x32;        
    }                     
    println(a);           
}                         
