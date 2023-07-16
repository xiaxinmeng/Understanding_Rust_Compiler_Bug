
$ rustc --test -O slice.rs                                                                                                                           
$ ./slice --bench

running 2 tests
test bench_heapsort_new ... bench:   5,890,135 ns/iter (+/- 102,715)
test bench_heapsort_old ... bench:   6,019,368 ns/iter (+/- 69,508)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

$ ./slice --bench

running 2 tests
test bench_heapsort_new ... bench:   5,876,645 ns/iter (+/- 88,551)
test bench_heapsort_old ... bench:   6,017,215 ns/iter (+/- 74,589)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

$ rustc -vV
rustc 1.18.0-nightly (ddc5d7bd4 2017-04-20)
binary: rustc
commit-hash: ddc5d7bd4b9ea3e8a8ccf82cb443e950be311d61
commit-date: 2017-04-20
host: x86_64-unknown-linux-gnu
release: 1.18.0-nightly
LLVM version: 3.9

