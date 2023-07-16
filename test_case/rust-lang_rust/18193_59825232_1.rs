
$ grep model\ name /proc/cpuinfo | uniq
model name  : Intel(R) Core(TM) i5-3317U CPU @ 1.70GHz
$ rustc -O --test test.rs
$ ./test --bench

running 5 tests
test _range    ... bench:     51362 ns/iter (+/- 4319)
test assembly  ... bench:     51578 ns/iter (+/- 2938)
test enumerate ... bench:     51465 ns/iter (+/- 850)
test iter      ... bench:     77122 ns/iter (+/- 886)
test position  ... bench:     77147 ns/iter (+/- 1315)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured
