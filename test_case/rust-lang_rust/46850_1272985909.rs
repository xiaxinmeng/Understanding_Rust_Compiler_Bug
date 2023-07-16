
$ qemu-aarch64 --version
qemu-aarch64 version 6.2.0 (Debian 1:6.2+dfsg-2ubuntu6.3)
Copyright (c) 2003-2021 Fabrice Bellard and the QEMU Project developers
$ rustc +nightly --version --verbose
rustc 1.66.0-nightly (8ce3204af 2022-09-30)
binary: rustc
commit-hash: 8ce3204af9463db3192ea1eb31c45c2f6d4b5ae6
commit-date: 2022-09-30
host: x86_64-unknown-linux-gnu
release: 1.66.0-nightly
LLVM version: 15.0.2
$ cargo +nightly bench --target aarch64-unknown-linux-gnu 
    Finished bench [optimized] target(s) in 0.00s
     Running unittests src/main.rs (target/aarch64-unknown-linux-gnu/release/deps/rust_46850-4e6b47a44975fd88)

running 1 test
test tests::bench_map_scalar ... bench:           1 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 0.37s

$ rustc +nightly-2017-12-18 --version --verbose
rustc 1.24.0-nightly (dc39c3169 2017-12-17)
binary: rustc
commit-hash: dc39c31699a83313edf2ac096d0bf3cef871b705
commit-date: 2017-12-17
host: x86_64-unknown-linux-gnu
release: 1.24.0-nightly
LLVM version: 4.0
$ cargo +nightly-2017-12-18 bench --target aarch64-unknown-linux-gnu 
warning: unused manifest key: package.edition
    Finished release [optimized] target(s) in 0.0 secs
     Running target/aarch64-unknown-linux-gnu/release/deps/rust_46850-03a8c8d252b37ebe

running 1 test
test tests::bench_map_scalar ... bench:           4 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out

