
$ time ./rustc-stage1 test.rs
error: main function not found

error: aborting due to previous error


real	0m0.041s
user	0m0.028s
sys	0m0.008s

$ time RUST_BACKTRACE=1 ./rustc-stage1 test.rs
error: main function not found

error: aborting due to previous error


real	0m0.153s
user	0m0.136s
sys	0m0.012s
