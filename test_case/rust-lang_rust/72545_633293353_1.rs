rust
// src/main.rs
use proc_rules::print_input;

macro_rules! with_ident {
     ($name: ident) => {
         #[print_input]
         pub fn $name() {}
     }
}

with_ident!(foo);

fn main() {}
