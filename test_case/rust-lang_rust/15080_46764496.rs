
#![feature(struct_variant)]

use std::string::{String};
use std::io::stdio::{println};

pub enum SuppressionType {
    MemcheckAddr(uint),
    MemcheckLeak,
    OtherType {
        pub tool_name: String,
        pub suppression_type: String,
    },
}

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
