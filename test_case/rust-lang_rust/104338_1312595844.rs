rust
#![feature(ptr_metadata)]
#![feature(dyn_star)]

use std::fmt::Debug;
use std::ptr::Thin;

fn polymorphic<T: Debug + ?Sized + Thin>(t: &T) {
    let _ = t as dyn* Debug;
}
