
% RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench -C lto=off  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,260 ns/iter (+/- 6)
test bench__vec_from_iter_using_baseline___ ... bench:       1,306 ns/iter (+/- 48)
test bench__vec_from_iter_with_filter_map__ ... bench:       3,467 ns/iter (+/- 3)
test bench__vec_from_iter_with_scan_freevar ... bench:       2,082 ns/iter (+/- 12)
test bench__vec_from_iter_with_scan_param__ ... bench:       2,087 ns/iter (+/- 52)
test bench_last_from_iter_using_adapter____ ... bench:       2,186 ns/iter (+/- 6)
test bench_last_from_iter_using_baseline___ ... bench:       2,187 ns/iter (+/- 21)
test bench_last_from_iter_with_filter_map__ ... bench:       2,650 ns/iter (+/- 3)
test bench_last_from_iter_with_scan_freevar ... bench:       2,683 ns/iter (+/- 4)
test bench_last_from_iter_with_scan_param__ ... bench:       2,688 ns/iter (+/- 8)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
