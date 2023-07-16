
$ rustc -vV
rustc 1.68.0-nightly (b70baa4f9 2022-12-14)
binary: rustc
commit-hash: b70baa4f922a1809d79caeaeb902800c3be283b9
commit-date: 2022-12-14
host: x86_64-unknown-linux-gnu
release: 1.68.0-nightly
LLVM version: 15.0.6

$ cargo bench
    Finished bench [optimized] target(s) in 0.14s
     Running unittests src/main.rs (target/release/deps/bykey-4069213d2c3527e4)

running 2 tests
test tests::bench_max_by_key  ... bench:       4,298 ns/iter (+/- 236)
test tests::bench_max_by_key2 ... bench:       4,370 ns/iter (+/- 721)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out; finished in 1.88s
