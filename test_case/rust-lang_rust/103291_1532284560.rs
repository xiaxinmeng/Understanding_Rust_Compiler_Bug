rust
#![feature(const_type_id)]

use std::any::TypeId;

const INT: TypeId = TypeId::of::<i32>();

fn main() {
    match TypeId::of::<u8>() {
        INT => println!("i32"),
        _ => println!("not i32"),
    }
}
