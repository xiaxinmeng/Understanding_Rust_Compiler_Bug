
$ mkdir ~/.rust
$ export RUST_PATH=$~/.rust
$ cd ~/dev/rust
$ git clone github.com/bjz/sax-rs
$ cd sax-rs/src/sax
$ rustpkg install
WARNING: The Rust package manager is experimental and may be unstable
note: Installed package sax-0.1 to /Users/brendan/.rust
$ cd ~/dev/rust
$ git clone github.com/bjz/gl-rs
$ cd gl-rs/src/gen
$ rustpkg install
WARNING: The Rust package manager is experimental and may be unstable
task '<unnamed>' failed at 'assertion failed: pkg_src.start_dir.join(p).exists()', /Users/brendan/dev/rust/rust/src/librustpkg/lib.rs:520
task '<unnamed>' failed at 'receiving on closed channel', /Users/brendan/dev/rust/rust/src/libstd/rt/comm.rs:198
