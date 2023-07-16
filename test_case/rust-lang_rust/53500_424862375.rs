rust
#![feature(convert_id)]
use std::convert::identity;

fn manipulation(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    let do_stuff = if true { manipulation } else { identity };
    let _results = do_stuff(1, 2);
}
