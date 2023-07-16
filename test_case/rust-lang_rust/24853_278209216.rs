
error[E0464]: multiple matching crates for `getopts`
 --> /home/siege/Projects/RustGnuplot/examples/common.rs:3:1
  |
3 | extern crate getopts;
  | ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: candidates:
  = note: path: /home/siege/Projects/RustGnuplot/build2/getopts_dep/target/debug/deps/libgetopts-3facdbd0235704b0.rlib
  = note: crate name: getopts
  = note: path: /usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-9f0027fbffa38971.so
  = note: path: /usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-9f0027fbffa38971.rlib
  = note: crate name: getopts
