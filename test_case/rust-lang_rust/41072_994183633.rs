console
$ cat Cargo.toml 
[package]
name = "flate2"
version = "0.0.0"
edition = "2021"

$ cat src/lib.rs 
mod gz {
    pub struct EncoderReader;

    impl EncoderReader {
        pub fn new() -> EncoderReader {
            unimplemented!()
        }
    }
}

pub use gz::EncoderReader as GzEncoder;

$ cargo doc --open
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Opening target/doc/flate2/index.html
