rust
#![feature(rustc_private)]

extern crate syntax_pos;
extern crate serialize;

use syntax_pos::FileMap;
use serialize::{opaque, Decodable};

fn main() {
    let bytes = [0, 0, 0, 0, 2, 0, 0];
    let _ = FileMap::decode(&mut opaque::Decoder::new(&bytes, 0));
}
