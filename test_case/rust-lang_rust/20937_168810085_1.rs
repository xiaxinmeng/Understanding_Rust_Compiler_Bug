
$ rustc --pretty=expanded hello.rs -Z unstable-options
hello.rs:2:29: 2:30 error: expected type, found `6`
hello.rs:2     y & CONT_MASK as u32 << 6;
                                       ^
