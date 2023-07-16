 bash
$ time RUST_BACKTRACE=1 /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/bin/rustc -VV
error: Option 'version' given more than once.

real    0m0.401s
user    0m0.360s
sys 0m0.020s

$ time RUST_BACKTRACE=1 /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/bin/rustc -Vv
rustc 1.4.0-dev (1af31d497 2015-08-11)
binary: rustc
commit-hash: 1af31d4974e33027a68126fa5a5a3c2c6491824f
commit-date: 2015-08-11
host: x86_64-unknown-linux-gnu
release: 1.4.0-dev

real    0m0.018s
user    0m0.000s
sys 0m0.010s
