
$ RUSTFLAGS='-Z trans-time-graph -Z thinlto -C codegen-units=16' cargo +nightly build --release
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling adler32 v1.0.2
   Compiling inflate v0.3.3 (file:///home/alex/code/inflate)
    Finished release [optimized] target(s) in 16.47 secs
