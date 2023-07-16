rust
#![feature(proc_macro)]

extern crate foo_macros;
use foo_macros::foo_attr;

mod bar {
    #![foo_attr]
}
