 rust
# Download the previous compiler, e.g. 1.12.0 from somewhere
$ curl https://sh.rustup.rs | sh -s -- --default-toolchain 1.12.0

# Configure rust to build with that compiler
$ ./configure --local-rust-root=$(rustc --print sysroot)
$ make
