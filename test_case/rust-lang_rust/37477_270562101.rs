
$ time rustc test.rs
error: main function not found

error: aborting due to previous error

real	0m0.061s
user	0m0.034s
sys	0m0.024s

$ time RUST_BACKTRACE=1 rustc test.rs
error: main function not found

error: aborting due to previous error

real	0m0.123s
user	0m0.061s
sys	0m0.031s
