bash
$ rustc --version --verbose
rustc 1.49.0 (e1884a8e3 2020-12-29)
binary: rustc
commit-hash: e1884a8e3c3e813aada8254edfa120e85bf5ffca
commit-date: 2020-12-29
host: x86_64-unknown-linux-gnu
release: 1.49.0
$ rustc -C llvm-args=--version
LLVM (http://llvm.org/):
  LLVM version 11.0.0-rust-1.49.0-stable
  Optimized build.
  Default target: x86_64-unknown-linux-gnu
  Host CPU: ivybridge
