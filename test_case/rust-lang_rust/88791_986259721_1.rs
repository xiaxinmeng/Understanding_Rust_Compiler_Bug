rust
//! /src/lib.rs

/// Hello from lib.
#[derive(Debug)]
pub struct Hello;

#[path = "../examples"]
#[cfg(any(doc, test))]
pub mod examples {
    #![allow(dead_code)]

    pub mod world;
}
