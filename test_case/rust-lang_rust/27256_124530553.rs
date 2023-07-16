 bash
$ sysctl machdep.cpu.brand_string
machdep.cpu.brand_string: Intel(R) Core(TM)2 Duo CPU     P8600  @ 2.40GHz
$ rustc -vV
rustc 1.3.0-nightly (0c052199b 2015-07-11)
binary: rustc
commit-hash: 0c052199b92104ba6d64886ff779cf89c3c384d9
commit-date: 2015-07-11
host: x86_64-apple-darwin
release: 1.3.0-nightly
$ rustc --test -O rot_bench.rs
$ ./rot_bench --bench

running 4 tests
test test_rotate_1 ... ignored
test test_rotate_16 ... ignored
test bench_rotate_16     ... bench:           8 ns/iter (+/- 1)
test bench_rotate_16_new ... bench:          13 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured
