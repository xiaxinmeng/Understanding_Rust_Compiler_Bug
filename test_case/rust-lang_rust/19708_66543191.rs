
$ ./a --bench

running 6 tests
test bench_long_direct       ... bench:    174465 ns/iter (+/- 15410) = 4332 MB/s
test bench_long_via_format   ... bench:    823062 ns/iter (+/- 16784) = 917 MB/s
test bench_medium_direct     ... bench:        54 ns/iter (+/- 1) = 13999 MB/s
test bench_medium_via_format ... bench:       827 ns/iter (+/- 17) = 914 MB/s
test bench_short_direct      ... bench:        30 ns/iter (+/- 1) = 399 MB/s
test bench_short_via_format  ... bench:       116 ns/iter (+/- 2) = 103 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured
