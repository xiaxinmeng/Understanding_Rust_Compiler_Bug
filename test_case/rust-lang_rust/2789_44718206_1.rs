
/**
 * My mod
 */

// For log related macros:
#![feature(phase)]
#[phase(syntax, link)] extern crate log;

fn main() {
  error!("We have logging!");
}
