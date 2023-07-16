rust
use std::process::Command;                                
                                                          
fn main() {                                               
    println!("{:?}", Command::new("nonexistent").spawn());
}                                                         
