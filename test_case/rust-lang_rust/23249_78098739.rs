
$ rustc bench.rs --test -O       
$ ./bench --bench 
running 2 tests
test bench::bench_new ... bench:        49 ns/iter (+/- 1) = 1326 MB/s
test bench::bench_old ... bench:        49 ns/iter (+/- 2) = 1326 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
