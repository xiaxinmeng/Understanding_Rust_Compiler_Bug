
$ time ./rustc-stage2 test.rs
error: main function not found

error: aborting due to previous error

real	0m0.041s
user	0m0.032s
sys	0m0.012s

$ time RUST_BACKTRACE=1 ./rustc-stage2 test.rs
error: main function not found

error: aborting due to previous error

real	0m0.157s
user	0m0.136s
sys	0m0.016s
