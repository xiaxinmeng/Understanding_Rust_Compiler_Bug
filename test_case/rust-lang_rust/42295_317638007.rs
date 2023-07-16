
$ rustc -V
rustc 1.19.0-nightly (258ae6dd9 2017-06-15)

$ time RUST_BACKTRACE=1 rustc -vv
error: Option 'verbose' given more than once.
real    0m0.762s
user    0m0.392s
sys     0m0.052s

$ time RUST_BACKTRACE=0 rustc -vv
error: Option 'verbose' given more than once.
real    0m0.231s
user    0m0.048s
sys     0m0.176s
