
11-54-00 Mozilla/issue11084 % RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=16 --test main-bench.rs -o main-bench -C lto=off  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       6,830 ns/iter (+/- 66)
test bench__vec_from_iter_using_baseline___ ... bench:       6,848 ns/iter (+/- 30)
test bench__vec_from_iter_with_filter_map__ ... bench:       5,547 ns/iter (+/- 20)
test bench__vec_from_iter_with_scan_freevar ... bench:       5,592 ns/iter (+/- 84)
test bench__vec_from_iter_with_scan_param__ ... bench:       5,322 ns/iter (+/- 12)
test bench_last_from_iter_using_adapter____ ... bench:       9,820 ns/iter (+/- 216)
test bench_last_from_iter_using_baseline___ ... bench:      14,730 ns/iter (+/- 116)
test bench_last_from_iter_with_filter_map__ ... bench:       9,869 ns/iter (+/- 16)
test bench_last_from_iter_with_scan_freevar ... bench:      11,351 ns/iter (+/- 131)
test bench_last_from_iter_with_scan_param__ ... bench:      10,593 ns/iter (+/- 51)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
