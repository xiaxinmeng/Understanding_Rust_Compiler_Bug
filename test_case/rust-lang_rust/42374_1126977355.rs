rust
#![feature(rustc_private)]

extern crate rustc_span;
extern crate rustc_serialize;

use rustc_span::SourceFile as FileMap;
use rustc_serialize::{opaque, Decodable};

fn main() {
    let bytes = [0, 0, 0, 0, 2, 0, 0];
    let _ = FileMap::decode(&mut opaque::Decoder::new(&bytes, 0));
}
