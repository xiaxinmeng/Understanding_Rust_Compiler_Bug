
$ rustc --test -O 1.rs

$ ./1 --bench

running 2 tests
test bench_heapsort_new ... bench:   6,505,975 ns/iter (+/- 805,094)
test bench_heapsort_old ... bench:   6,484,118 ns/iter (+/- 592,318)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

$ ./1 --bench

running 2 tests
test bench_heapsort_new ... bench:   6,365,943 ns/iter (+/- 470,100)
test bench_heapsort_old ... bench:   6,501,713 ns/iter (+/- 725,469)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

$ rustc -vV
rustc 1.18.0-nightly (ddc5d7bd4 2017-04-20)
binary: rustc
commit-hash: ddc5d7bd4b9ea3e8a8ccf82cb443e950be311d61
commit-date: 2017-04-20
host: x86_64-apple-darwin
release: 1.18.0-nightly
LLVM version: 3.9
