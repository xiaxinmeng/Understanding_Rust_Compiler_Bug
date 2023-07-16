 bash
$ time rustc -vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once.

real    0m0.058s
user    0m0.021s
sys 0m0.035s

$ time rustc -vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once.

real    0m0.058s
user    0m0.020s
sys 0m0.037s

$ time RUST_BACKTRACE=1 rustc -vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once.

real    0m0.372s
user    0m0.320s
sys 0m0.050s

$ time RUST_BACKTRACE=1 rustc -vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once.

real    0m0.371s
user    0m0.315s
sys 0m0.053s

$ rustc -Vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-Vv'
rustc 1.11.0-dev (8d8a88f4a 2016-06-19)
binary: rustc
commit-hash: 8d8a88f4a5bac0038a621bfb195db13d9ae00cb8
commit-date: 2016-06-19
host: x86_64-unknown-linux-gnu
release: 1.11.0-dev

