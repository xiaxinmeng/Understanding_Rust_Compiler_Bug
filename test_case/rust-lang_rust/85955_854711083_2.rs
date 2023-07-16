rust
extern crate abc; // abc must be replaced by the name of the crate to test (e.g. the name in the Cargo.toml file)

include!(concat!(env!("OUT_DIR"), "/", "crash.rs"));
