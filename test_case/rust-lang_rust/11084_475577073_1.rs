
11-54-19 Mozilla/issue11084 % RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=16 --test main-bench.rs -o main-bench -C lto=fat  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,914 ns/iter (+/- 12)
test bench__vec_from_iter_using_baseline___ ... bench:       1,914 ns/iter (+/- 2)
test bench__vec_from_iter_with_filter_map__ ... bench:       3,406 ns/iter (+/- 10)
test bench__vec_from_iter_with_scan_freevar ... bench:       1,354 ns/iter (+/- 4)
test bench__vec_from_iter_with_scan_param__ ... bench:       1,340 ns/iter (+/- 7)
test bench_last_from_iter_using_adapter____ ... bench:       1,648 ns/iter (+/- 17)
test bench_last_from_iter_using_baseline___ ... bench:       1,607 ns/iter (+/- 11)
test bench_last_from_iter_with_filter_map__ ... bench:       8,309 ns/iter (+/- 23)
test bench_last_from_iter_with_scan_freevar ... bench:       1,648 ns/iter (+/- 8)
test bench_last_from_iter_with_scan_param__ ... bench:       1,648 ns/iter (+/- 14)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
