 bash
$ time RUST_BACKTRACE= /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc -VV 
error: Option 'version' given more than once.

real    0m2.671s
user    0m2.407s
sys 0m0.070s

$ time RUST_BACKTRACE= /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc -Vv
rustc 1.5.0-dev (2a418216f 2015-10-25)
binary: rustc
commit-hash: 2a418216feeb83fd3f68c725c0e5577beacff59b
commit-date: 2015-10-25
host: x86_64-unknown-linux-gnu
release: 1.5.0-dev

real    0m0.123s
user    0m0.080s
sys 0m0.030s
