 rust
#![feature(const_fn)]

use std::fmt::Debug;
use std::cell::UnsafeCell;

#[derive(Debug)]
struct Doop(UnsafeCell<u32>);

unsafe impl Send for Doop {} 
unsafe impl Sync for Doop {} 

static ASDF_INNER: Doop = Doop(UnsafeCell::new(1));
//static ASDF: &'static (Debug + Sync) = &ASDF_INNER; 
  const ASDF: &'static (Debug + Sync) = &ASDF_INNER; 

fn main() {
    println!("{:#?}", ASDF);
}
