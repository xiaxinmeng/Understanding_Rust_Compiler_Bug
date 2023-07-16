shell
$ time ~/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Zinstrument-coverage -Copt-level=1 brotli_covtest.rs -L ~/rust-brotli-decompressor/target/debug/ -L ~/rust-brotli-decompressor/target/debug/deps --edition=2018

real    0m1.111s
user    0m3.136s
sys     0m0.179s

$ time ~/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Zinstrument-coverage -Copt-level=2 brotli_covtest.rs -L ~/rust-brotli-decompressor/target/debug/ -L ~/rust-brotli-decompressor/target/debug/deps --edition=2018

real    0m23.503s
user    0m36.125s
sys     0m0.181s

$ time ~/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Zinstrument-coverage -Copt-level=3 brotli_covtest.rs -L ~/rust-brotli-decompressor/target/debug/ -L ~/rust-brotli-decompressor/target/debug/deps --edition=2018

real    0m23.822s
user    0m36.961s
sys     0m0.211s
