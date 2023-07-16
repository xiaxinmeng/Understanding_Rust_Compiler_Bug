
$ git clone https://github.com/nickel-org/nickel.rs.git
$ cd nickel.rs
$ git checkout 0.8.0
$ cargo build
$ cargo clean -p nickel
$ cargo rustc -- -Z time-passes | grep LLVM
time: 0.740; rss: 144MB LLVM passes
$ cargo clean -p nickel
$ cargo rustc -- -Z orbit -Z time-passes | grep LLVM
time: 5.738; rss: 146MB LLVM passes
