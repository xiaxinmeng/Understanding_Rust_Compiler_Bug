
% RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench -C lto=fat  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,289 ns/iter (+/- 19)
test bench__vec_from_iter_using_baseline___ ... bench:       1,232 ns/iter (+/- 9)
test bench__vec_from_iter_with_filter_map__ ... bench:       3,481 ns/iter (+/- 19)
test bench__vec_from_iter_with_scan_freevar ... bench:       1,273 ns/iter (+/- 20)
test bench__vec_from_iter_with_scan_param__ ... bench:       1,273 ns/iter (+/- 8)
test bench_last_from_iter_using_adapter____ ... bench:       1,680 ns/iter (+/- 18)
test bench_last_from_iter_using_baseline___ ... bench:       1,683 ns/iter (+/- 21)
test bench_last_from_iter_with_filter_map__ ... bench:       3,047 ns/iter (+/- 2)
test bench_last_from_iter_with_scan_freevar ... bench:       1,793 ns/iter (+/- 81)
test bench_last_from_iter_with_scan_param__ ... bench:       1,773 ns/iter (+/- 49)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
