 bash
wget http://static.rust-lang.org/dist/2015-02-14/rustc-nightly-x86_64-unknown-linux-gnu.tar.gz
tar -xf rustc-nightly-x86_64-unknown-linux-gnu.tar.gz
wget http://static.rust-lang.org/dist/2015-02-08/rustc-nightly-i686-unknown-linux-gnu.tar.gz
tar -xf rustc-nightly-i686-unknown-linux-gnu.tar.gz

RUST_BACKTRACE=1 LD_LIBRARY_PATH="rustc-nightly-x86_64-unknown-linux-gnu/lib" ./rustc-nightly-x86_64-unknown-linux-gnu/bin/rustc --target=i686-unknown-linux-gnu -L rustc-nightly-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/ hello.rs
