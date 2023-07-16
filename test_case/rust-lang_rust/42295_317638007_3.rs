
$ rustup run stable rustc -V
rustc 1.18.0 (03fc9d622 2017-06-06)

$ time RUST_BACKTRACE=1 rustup run stable rustc -vv
error: Option 'verbose' given more than once.
real    0m0.439s
user    0m0.288s
sys     0m0.144s

$ time RUST_BACKTRACE=0 rustup run stable rustc -vv
error: Option 'verbose' given more than once.
real    0m0.056s
user    0m0.036s
sys     0m0.012s

$ time RUST_BACKTRACE=1 ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc -vv
error: Option 'verbose' given more than once.
real    0m0.260s
user    0m0.164s
sys     0m0.092s

$ time RUST_BACKTRACE=0 ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc -vv
error: Option 'verbose' given more than once.
real    0m0.106s
user    0m0.032s
sys     0m0.068s
