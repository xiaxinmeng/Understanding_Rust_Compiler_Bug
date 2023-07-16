 rust
use std::any::Any;

fn main() {
    let weird : [([u8; 188250],[u8; 1381155],[u8; 558782]); 0] = [];
    let whoops = Any::downcast_ref::<[([u8; 1990233],[u8; 798602],[u8; 2074279]); 1]>(&weird);
    println!("{}",whoops.unwrap()[0].0[333333]);
}
