
$ time RUST_BACKTRACE=1 rustup run nightly-2017-05-28 rustc -vv
error: Option 'verbose' given more than once.
real    0m0.339s
user    0m0.324s
sys     0m0.012s

$ time RUST_BACKTRACE=0 rustup run nightly-2017-05-28 rustc -vv
error: Option 'verbose' given more than once.
real    0m0.063s
user    0m0.048s
sys     0m0.008s
