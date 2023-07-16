
$ rustc --version
rustc 0.11.0-pre-nightly (7580ef9 2014-06-08 00:46:57 -0700)
host: x86_64-unknown-linux-gnu
$ rusct issue-13446.rs:issue-13446.rs:1:28: 1:35 error: mismatched types: expected `[u32, .. 256]` but found `collections::vec::Vec<<generic #2>>` (expected vector but found struct collections::vec::Vec)
issue-13446.rs:1 static VEC: [u32, ..256] = vec!();
                                            ^~~~~~~
error: aborting due to previous error
