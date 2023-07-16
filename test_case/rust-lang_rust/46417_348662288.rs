rust
extern crate glob;

use glob::Pattern;

fn main() {
    let pat = Pattern::new("{*").unwrap();
    println!("{:?}", pat.matches("{foo"));
    
    let pat = Pattern::new("{foo}").unwrap();
    println!("{:?}", pat.matches("{foo}"));
}
