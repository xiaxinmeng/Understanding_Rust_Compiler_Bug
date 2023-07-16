
12-45-10 Mozilla/issue11084 % rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench
12-45-45 Mozilla/issue11084 % ./main-bench --bench

running 5 tests
test bench_from_iter_using_adapter____ ... bench:       1,531 ns/iter (+/- 6)
test bench_from_iter_using_baseline___ ... bench:       1,277 ns/iter (+/- 10)
test bench_from_iter_with_filter_map__ ... bench:       3,508 ns/iter (+/- 64)
test bench_from_iter_with_scan_freevar ... bench:       2,066 ns/iter (+/- 53)
test bench_from_iter_with_scan_param__ ... bench:       2,055 ns/iter (+/- 30)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out

12-45-51 Mozilla/issue11084 % ./main-bench --bench

running 5 tests
test bench_from_iter_using_adapter____ ... bench:       1,514 ns/iter (+/- 6)
test bench_from_iter_using_baseline___ ... bench:       1,305 ns/iter (+/- 12)
test bench_from_iter_with_filter_map__ ... bench:       3,491 ns/iter (+/- 56)
test bench_from_iter_with_scan_freevar ... bench:       2,061 ns/iter (+/- 5)
test bench_from_iter_with_scan_param__ ... bench:       2,094 ns/iter (+/- 53)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out

12-45-53 Mozilla/issue11084 % ./main-bench --bench adapter

running 1 test
test bench_from_iter_using_adapter____ ... bench:       1,574 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 4 filtered out

12-45-57 Mozilla/issue11084 % ./main-bench --bench adapter

running 1 test
test bench_from_iter_using_adapter____ ... bench:       1,530 ns/iter (+/- 9)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 4 filtered out

12-45-59 Mozilla/issue11084 % ./main-bench --bench baseline

running 1 test
test bench_from_iter_using_baseline___ ... bench:       1,569 ns/iter (+/- 9)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 4 filtered out

12-46-01 Mozilla/issue11084 % ./main-bench --bench baseline

running 1 test
test bench_from_iter_using_baseline___ ... bench:       1,576 ns/iter (+/- 4)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 4 filtered out

12-46-03 Mozilla/issue11084 %
