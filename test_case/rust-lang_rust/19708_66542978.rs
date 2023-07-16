
$ ./a --bench

running 6 tests
test bench_long_direct       ... bench:  42474438 ns/iter (+/- 818109) = 17 MB/s
test bench_long_via_format   ... bench:   9322190 ns/iter (+/- 388832) = 80 MB/s
test bench_medium_direct     ... bench:     42862 ns/iter (+/- 1327) = 17 MB/s
test bench_medium_via_format ... bench:      9349 ns/iter (+/- 282) = 80 MB/s
test bench_short_direct      ... bench:       871 ns/iter (+/- 29) = 13 MB/s
test bench_short_via_format  ... bench:       344 ns/iter (+/- 9) = 34 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured
