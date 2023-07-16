 rust
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;

extern crate image;

use std::io::BufRead;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Action {
    Undo,
    Redo,
    Crop {x: u32, y: u32, width: u32, height: u32},
    FlipHorizontal,
    FlipVertical,
    Resize {width: u32, height: u32},
}
