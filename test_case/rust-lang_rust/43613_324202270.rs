
$ rustc -V
rustc 1.19.0 (0ade33941 2017-07-17)
$ time rustc foo.rs
rustc foo.rs  3.00s user 0.02s system 99% cpu 3.040 total
$ rustc +beta -V
rustc 1.20.0-beta.2 (54279dfac 2017-08-13)
$ time rustc +beta foo.rs
rustc +beta foo.rs  6.17s user 1.26s system 99% cpu 7.437 total
