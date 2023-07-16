 bash
$ rustc -vV
rustc 1.3.0-nightly (69ca01256 2015-07-23)
binary: rustc
commit-hash: 69ca0125641db798f072f9a0f5d838686255eb37
commit-date: 2015-07-23
host: x86_64-apple-darwin
release: 1.3.0-nightly
$ rustc --test -O rot_bench.rs
$ ./rot_bench --bench

running 4 tests
test test_rotate_1 ... ignored
test test_rotate_16 ... ignored
test bench_rotate_16     ... bench:           8 ns/iter (+/- 1)
test bench_rotate_16_new ... bench:           8 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured
