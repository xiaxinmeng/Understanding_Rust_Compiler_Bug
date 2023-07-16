
% RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench -C lto=thin  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,263 ns/iter (+/- 8)
test bench__vec_from_iter_using_baseline___ ... bench:       1,238 ns/iter (+/- 49)
test bench__vec_from_iter_with_filter_map__ ... bench:       3,702 ns/iter (+/- 32)
test bench__vec_from_iter_with_scan_freevar ... bench:       1,248 ns/iter (+/- 3)
test bench__vec_from_iter_with_scan_param__ ... bench:       1,248 ns/iter (+/- 13)
test bench_last_from_iter_using_adapter____ ... bench:       1,616 ns/iter (+/- 6)
test bench_last_from_iter_using_baseline___ ... bench:       1,612 ns/iter (+/- 6)
test bench_last_from_iter_with_filter_map__ ... bench:       3,024 ns/iter (+/- 7)
test bench_last_from_iter_with_scan_freevar ... bench:       1,427 ns/iter (+/- 7)
test bench_last_from_iter_with_scan_param__ ... bench:       1,427 ns/iter (+/- 11)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
