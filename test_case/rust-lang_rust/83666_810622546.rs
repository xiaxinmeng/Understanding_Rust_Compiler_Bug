rust
extern crate brotli_decompressor;
use std::io;

fn main() {
    match brotli_decompressor::BrotliDecompress(&mut io::stdin(), &mut io::stdout()) {
        Ok(_) => {}
        Err(e) => println!("Error {:?}", e),
    }
}
