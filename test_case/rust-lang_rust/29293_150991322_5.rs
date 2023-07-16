 bash
$ time RUST_BACKTRACE= /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV
error: Option 'version' given more than once.

real    0m46.847s
user    0m45.843s
sys 0m0.313s

$ time RUST_BACKTRACE= /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -Vv
rustc 1.5.0-dev (2a418216f 2015-10-25)
binary: rustc
commit-hash: 2a418216feeb83fd3f68c725c0e5577beacff59b
commit-date: 2015-10-25
host: x86_64-unknown-linux-gnu
release: 1.5.0-dev

real    0m0.181s
user    0m0.080s
sys 0m0.030s

$ time /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV
error: Option 'version' given more than once.

real    0m0.116s
user    0m0.083s
sys 0m0.033s

