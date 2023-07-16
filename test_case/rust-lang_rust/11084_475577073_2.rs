
11-55-02 Mozilla/issue11084 % RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=16 --test main-bench.rs -o main-bench -C lto=thin  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       2,076 ns/iter (+/- 82)
test bench__vec_from_iter_using_baseline___ ... bench:       2,076 ns/iter (+/- 3)
test bench__vec_from_iter_with_filter_map__ ... bench:       3,792 ns/iter (+/- 11)
test bench__vec_from_iter_with_scan_freevar ... bench:       6,046 ns/iter (+/- 21)
test bench__vec_from_iter_with_scan_param__ ... bench:       6,064 ns/iter (+/- 46)
test bench_last_from_iter_using_adapter____ ... bench:       1,429 ns/iter (+/- 12)
test bench_last_from_iter_using_baseline___ ... bench:       1,501 ns/iter (+/- 21)
test bench_last_from_iter_with_filter_map__ ... bench:       8,945 ns/iter (+/- 91)
test bench_last_from_iter_with_scan_freevar ... bench:      15,404 ns/iter (+/- 15)
test bench_last_from_iter_with_scan_param__ ... bench:      15,385 ns/iter (+/- 17)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
