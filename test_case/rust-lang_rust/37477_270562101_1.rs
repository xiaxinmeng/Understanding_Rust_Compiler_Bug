
$ time rustc test.rs
error: main function not found

error: aborting due to previous error

real	0m0.045s
user	0m0.036s
sys	0m0.000s

$ time RUST_BACKTRACE=1 rustc test.rs
error: main function not found

error: aborting due to previous error

real	0m10.737s
user	0m10.592s
sys	0m0.032s
