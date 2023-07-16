
#![feature(old_io)]
#![feature(fs)]
#![allow(unused_must_use)]

use std::fs;
use std::old_io as io;

fn main() {
  io::println("Hello");
  fs::create_dir("repr");
}
