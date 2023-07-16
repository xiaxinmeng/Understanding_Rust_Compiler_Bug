 bash
$ rustc --version
rustc 1.14.0-nightly (16eeeac78 2016-10-18)
$ rustc prime.rs -O --crate-type cdylib
$ nm libprime.rs
00000000000183b0 T is_prime
