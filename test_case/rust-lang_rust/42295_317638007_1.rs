
$ time RUST_BACKTRACE=1 ~/.rustup/toolchains/nightly-2017-05-28-x86_64-unknown-linux-gnu/bin/rustc -vv
error: Option 'verbose' given more than once.
real    0m0.263s
user    0m0.236s
sys     0m0.020s

$ time RUST_BACKTRACE=0 ~/.rustup/toolchains/nightly-2017-05-28-x86_64-unknown-linux-gnu/bin/rustc -vv
error: Option 'verbose' given more than once.
real    0m0.060s
user    0m0.056s
sys     0m0.004s
