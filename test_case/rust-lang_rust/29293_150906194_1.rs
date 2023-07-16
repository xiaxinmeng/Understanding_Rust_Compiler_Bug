
$ time RUST_BACKTRACE=0 /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV
error: Option 'version' given more than once.

real    0m44.758s
user    0m44.310s
sys 0m0.390s

$ time RUST_BACKTRACE= /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV
error: Option 'version' given more than once.

real    0m44.767s
user    0m44.460s
sys 0m0.283s
