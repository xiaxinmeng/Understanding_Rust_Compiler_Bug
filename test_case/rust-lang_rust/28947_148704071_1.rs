 bash
$ time x86_64-unknown-linux-gnu/stage0/bin/rustc -g --emit llvm-bc src/libsyntax/lib.rs
real    1m23.926s
user    1m20.543s
sys 0m3.040s

$ time opt -O2 -disable-output syntax.bc
real    4m24.718s
user    4m23.393s
sys 0m0.653s
//oh wait, what? this worked this time?! WHAT? :) without -verify-each and with --enable-debuginfo that was left from above.
//I don't get it; is it my 3.8.0 llc ? I don't remember if I used this llc version to test this
...
