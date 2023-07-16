 rust
// rustc -o main -L lib main.rs

#![feature(struct_variant)]

extern crate mylib;

use mylib::{SuppressionType, MemcheckAddr, MemcheckLeak, OtherType};
use std::io::stdio::{println};
use std::string::{String};

fn do_stuff(t: &SuppressionType) {
    let s = match t {
            &MemcheckAddr(n) => format!("Memcheck:Addr{:u}", n),
            &MemcheckLeak => format!("Memcheck:Leak"),
            &OtherType {
                tool_name: ref tool_name,
                suppression_type: ref suppression_type,
            } => {
                format!("{}:{}", tool_name.as_slice(), suppression_type.as_slice())
            },
        };
    println(s.as_slice());
}

fn main() {
    let t = MemcheckAddr(14);
    do_stuff(&t);
}
