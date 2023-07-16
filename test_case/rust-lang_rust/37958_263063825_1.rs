rust
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
pub struct Point {
    x: f64,
    y: f64,
}
