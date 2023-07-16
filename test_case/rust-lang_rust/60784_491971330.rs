rust
use std::mem::size_of_val;

fn main() {
    let mut range = 0..10;
    println!("range: {}", size_of_val(&range));
    
    let mut iter: &mut dyn Iterator<Item = i32> = &mut range;
    println!("iter:  {}", size_of_val(iter));
    
    let iter2: &mut dyn Iterator<Item = i32> = &mut iter;
    println!("iter2: {}", size_of_val(iter2));
}
