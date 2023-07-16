 rust
#![feature(const_fn)]
#![feature(thread_local)]                

use std::cell::Cell;                     

#[thread_local]                          
pub static FOO: Cell<u32> = Cell::new(3);

fn main() {}
