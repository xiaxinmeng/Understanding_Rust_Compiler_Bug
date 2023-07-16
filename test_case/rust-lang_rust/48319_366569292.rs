
$ ./cargo-curl failure 0.1.1
$ cd failure-0.1.1/
failure-0.1.1$ cargo +stable build --release
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling cc v1.0.4
   Compiling cfg-if v0.1.2
   Compiling unicode-xid v0.0.4
   Compiling quote v0.3.15
   Compiling rustc-demangle v0.1.6
   Compiling libc v0.2.36
   Compiling synom v0.11.3
   Compiling syn v0.11.11
   Compiling backtrace-sys v0.1.16
   Compiling synstructure v0.6.1
   Compiling failure_derive v0.1.1
   Compiling backtrace v0.3.5
   Compiling failure v0.1.1 (...)
    Finished release [optimized] target(s) in 11.60 secs
failure-0.1.1$ cargo +stable build --release
    Finished release [optimized] target(s) in 0.0 secs
failure-0.1.1$ touch src/lib.rs
failure-0.1.1$ cargo +stable build --release
   Compiling failure v0.1.1 (...)
    Finished release [optimized] target(s) in 0.36 secs
