 bash
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 'RUSTFLAGS=--verbose -Z verbose' RUST_BACKTRACE=1
...
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front] Error 101

real    6m31.235s
user    6m27.830s
sys 0m2.010s
