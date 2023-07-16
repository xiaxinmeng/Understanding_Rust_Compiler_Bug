
$ rustc -Copt-level=3 -C target-cpu=corei7-avx --test test.rs
$ ./test --bench

running 2 tests
test folds1 ... bench:          40 ns/iter (+/- 0)
test folds2 ... bench:          40 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
