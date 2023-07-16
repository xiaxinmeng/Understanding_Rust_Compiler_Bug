 rust
// mkdir -p lib
// rustc --out-dir lib mylib.rs

#![crate_id = "mylib#0.1"]
#![crate_type = "lib"]
#![feature(struct_variant)]

use std::string::{String};

pub enum SuppressionType {
    MemcheckAddr(uint),
    MemcheckLeak,
    OtherType {
        pub tool_name: String,
        pub suppression_type: String,
    },
}
