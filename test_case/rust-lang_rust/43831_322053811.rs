
[01:13:16]      Running build/x86_64-unknown-linux-gnu/stage2-test/i686-unknown-linux-musl/release/deps/test-e83a411296b8a083
[01:13:16] 
[01:13:16] running 36 tests
[01:13:16] test stats::bench::no_iter ... ok
[01:13:16] test should_sort_failures_before_printing_them ... ok
[01:13:16] test stats::bench::sum_three_items ... ok
[01:13:16] test stats::tests::test_binom25 ... ok
[01:13:16] test stats::bench::sum_many_f64 ... ok
[01:13:16] test stats::tests::test_exp10a ... ok
[01:13:16] test stats::tests::test_exp10b ... ok
[01:13:16] test stats::tests::test_exp10c ... ok
[01:13:16] test stats::tests::test_exp25 ... ok
[01:13:16] test stats::tests::test_min_max_nan ... ok
[01:13:16] test stats::tests::test_norm10medium ... ok
[01:13:16] test stats::tests::test_norm10narrow ... ok
[01:13:16] test stats::tests::test_norm10wide ... ok
[01:13:16] test stats::tests::test_norm2 ... ok
[01:13:16] test stats::tests::test_norm25verynarrow ... ok
[01:13:16] test stats::tests::test_pois25lambda30 ... ok
[01:13:16] test stats::tests::test_pois25lambda40 ... ok
[01:13:16] test stats::tests::test_sum_f64s ... ok
[01:13:16] test stats::tests::test_sum_f64_between_ints_that_sum_to_0 ... ok
[01:13:16] test stats::tests::test_pois25lambda50 ... ok
[01:13:16] test stats::tests::test_unif25 ... ok
[01:13:16] test tests::do_not_run_ignored_tests ... ok
[01:13:16] test tests::exact_filter_match ... ok
[01:13:16] test tests::filter_for_ignored_option ... ok
[01:13:16] test tests::ignored_tests_result_in_ignored ... ok
[01:13:16] test tests::parse_ignored_flag ... ok
[01:13:16] test tests::test_bench_no_iter ... ok
[01:13:16] test tests::sort_tests ... ok
[01:13:16] test tests::test_bench_once_no_iter ... ok
[01:13:16] test tests::test_metricmap_compare ... ok
[01:13:16] test tests::test_should_panic ... ok
[01:13:16] error: An unknown error occurred
[01:13:16] 
[01:13:16] To learn more, run the command again with --verbose.
[01:13:16] 
[01:13:16] 
[01:13:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "i686-unknown-linux-musl" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--features" "" "-p" "test:0.0.0" "-p" "term:0.0.0" "--"
[01:13:16] expected success, got: exit code: 101
[01:13:16] 
[01:13:16] 
[01:13:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i686-unknown-linux-musl --target i586-unknown-linux-gnu
[01:13:16] Build completed unsuccessfully in 1:11:59
[01:13:16] 
