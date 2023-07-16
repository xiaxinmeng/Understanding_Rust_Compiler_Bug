
% RUSTC_BOOTSTRAP=1 rustc +1.13.0 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,692 ns/iter (+/- 8)
test bench__vec_from_iter_using_baseline___ ... bench:       1,692 ns/iter (+/- 67)
test bench__vec_from_iter_with_filter_map__ ... bench:       1,963 ns/iter (+/- 10)
test bench__vec_from_iter_with_scan_freevar ... bench:       1,692 ns/iter (+/- 11)
test bench__vec_from_iter_with_scan_param__ ... bench:       1,690 ns/iter (+/- 8)
test bench_last_from_iter_using_adapter____ ... bench:       2,156 ns/iter (+/- 10)
test bench_last_from_iter_using_baseline___ ... bench:       2,158 ns/iter (+/- 8)
test bench_last_from_iter_with_filter_map__ ... bench:       3,022 ns/iter (+/- 8)
test bench_last_from_iter_with_scan_freevar ... bench:       2,156 ns/iter (+/- 12)
test bench_last_from_iter_with_scan_param__ ... bench:       2,155 ns/iter (+/- 6)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured

% RUSTC_BOOTSTRAP=1 rustc +1.33.0 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,211 ns/iter (+/- 3)
test bench__vec_from_iter_using_baseline___ ... bench:       1,280 ns/iter (+/- 4)
test bench__vec_from_iter_with_filter_map__ ... bench:       1,504 ns/iter (+/- 4)
test bench__vec_from_iter_with_scan_freevar ... bench:       2,088 ns/iter (+/- 61)
test bench__vec_from_iter_with_scan_param__ ... bench:       2,054 ns/iter (+/- 5)
test bench_last_from_iter_using_adapter____ ... bench:       2,186 ns/iter (+/- 5)
test bench_last_from_iter_using_baseline___ ... bench:       2,186 ns/iter (+/- 19)
test bench_last_from_iter_with_filter_map__ ... bench:       1,659 ns/iter (+/- 12)
test bench_last_from_iter_with_scan_freevar ... bench:       2,692 ns/iter (+/- 2)
test bench_last_from_iter_with_scan_param__ ... bench:       2,693 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out

% RUSTC_BOOTSTRAP=1 rustc +nightly-2019-03-20 -C opt-level=3 -C codegen-units=1 --test main-bench.rs -o main-bench  && ./main-bench --bench

running 10 tests
test bench__vec_from_iter_using_adapter____ ... bench:       1,270 ns/iter (+/- 53)
test bench__vec_from_iter_using_baseline___ ... bench:       1,265 ns/iter (+/- 54)
test bench__vec_from_iter_with_filter_map__ ... bench:       3,469 ns/iter (+/- 7)
test bench__vec_from_iter_with_scan_freevar ... bench:       2,076 ns/iter (+/- 1)
test bench__vec_from_iter_with_scan_param__ ... bench:       2,128 ns/iter (+/- 47)
test bench_last_from_iter_using_adapter____ ... bench:       2,186 ns/iter (+/- 5)
test bench_last_from_iter_using_baseline___ ... bench:       2,185 ns/iter (+/- 57)
test bench_last_from_iter_with_filter_map__ ... bench:       2,650 ns/iter (+/- 13)
test bench_last_from_iter_with_scan_freevar ... bench:       2,678 ns/iter (+/- 12)
test bench_last_from_iter_with_scan_param__ ... bench:       2,674 ns/iter (+/- 15)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
