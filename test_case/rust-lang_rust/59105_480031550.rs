rust
Checking test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking rustc_term v0.0.1
    Checking getopts v0.2.17
    Checking proc_macro v0.0.0 (/home/lzutao/github.com/lzutao/rust-fork/rust/src/libproc_macro)
    Checking libtest v0.0.1
error[E0460]: found possibly newer version of crate `std` which `getopts` depends onest
  --> /home/lzutao/.cargo/registry/src/github.com-1ecc6299db9ec823/libtest-0.0.1/lib.rs:34:5
   |
34 | use getopts;
   |     ^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/lzutao/github.com/lzutao/rust-fork/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-d26c9396879f81c2.rmeta
           crate `getopts`: /home/lzutao/github.com/lzutao/rust-fork/rust/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-267ef145c76bc717.rmeta

error: aborting due to previous error
