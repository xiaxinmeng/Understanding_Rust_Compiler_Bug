 bash
$ unset RUST_BACKTRACE

$ which rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc

$ time  rustc -VV
error: Option 'version' given more than once.

real    0m0.055s
user    0m0.027s
sys 0m0.023s

$ time RUST_BACKTRACE=1 rustc -VV
error: Option 'version' given more than once.

real    0m0.619s
user    0m0.563s
sys 0m0.050s

$ time RUST_BACKTRACE=1 rustc -Vv
rustc 1.6.0-dev (cc8d398e2 2015-10-30)
binary: rustc
commit-hash: cc8d398e28b6b1918ef85479c2d040dfd0fe582d
commit-date: 2015-10-30
host: x86_64-unknown-linux-gnu
release: 1.6.0-dev

real    0m0.055s
user    0m0.027s
sys 0m0.027s
