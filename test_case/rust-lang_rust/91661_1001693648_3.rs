
> cargo +stage1 test --tests
    Finished test [unoptimized + debuginfo] target(s) in 2.13s
     Running unittests (target\debug\deps\rand-c30b5860dafe0096.exe)

running 72 tests
test distributions::bernoulli::test::test_trivial ... ok
test distributions::bernoulli::test::value_stability ... ok
test distributions::distribution::tests::test_distributions_iter ... ok
test distributions::distribution::tests::test_dist_string ... ok
test distributions::distribution::tests::test_distributions_map ... ok
test distributions::distribution::tests::test_make_an_iter ... ok
test distributions::float::tests::f32_edge_cases ... ok
test distributions::float::tests::f64_edge_cases ... ok
test distributions::float::tests::value_stability ... ok
test distributions::integer::tests::test_integers ... ok
test distributions::integer::tests::value_stability ... ok
test distributions::other::tests::test_alphanumeric ... ok
test distributions::other::tests::test_chars ... ok
test distributions::other::tests::test_misc ... ok
test distributions::other::tests::value_stability ... ok
test distributions::uniform::tests::test_char ... ok
test distributions::uniform::tests::test_custom_uniform ... ok
test distributions::uniform::tests::test_float_overflow - should panic ... ok
test distributions::uniform::tests::test_float_overflow_single - should panic ... ok
test distributions::uniform::tests::test_durations ... ok
test distributions::uniform::tests::test_float_assertions ... ok
test distributions::uniform::tests::test_uniform_bad_limits_equal_int - should panic ... ok
test distributions::uniform::tests::test_uniform_bad_limits_flipped_int - should panic ... ok
test distributions::uniform::tests::test_uniform_from_std_range ... ok
test distributions::uniform::tests::test_uniform_from_std_range_inclusive ... ok
test distributions::uniform::tests::test_uniform_good_limits_equal_int ... ok
test distributions::uniform::tests::test_floats ... ok
test distributions::uniform::tests::value_stability ... ok
test distributions::weighted_index::test::test_accepting_nan ... ok
test distributions::weighted_index::test::test_update_weights ... ok
test distributions::weighted_index::test::value_stability ... ok
test rng::test::test_fill ... ok
test rng::test::test_fill_bytes_default ... ok
test rng::test::test_fill_empty ... ok
test rng::test::test_gen_bool ... ok
test rng::test::test_gen_range_panic_int - should panic ... ok
test rng::test::test_gen_range_panic_usize - should panic ... ok
test rng::test::test_rng_boxed_trait ... ok
test rng::test::test_rng_trait_object ... ok
test rngs::adapter::read::test::test_reader_rng_fill_bytes ... ok
test rngs::adapter::read::test::test_reader_rng_insufficient_bytes ... ok
test rngs::adapter::read::test::test_reader_rng_u32 ... ok
test rngs::adapter::read::test::test_reader_rng_u64 ... ok
test rngs::adapter::reseeding::test::test_clone_reseeding ... ok
test rng::test::test_gen_range_float ... ok
test rngs::adapter::reseeding::test::test_reseeding ... ok
test rngs::std::test::test_stdrng_construction ... ok
test rngs::thread::test::test_thread_rng ... ok
test seq::index::test::test_sample_boundaries ... ok
test rng::test::test_gen_range_int ... ok
test seq::index::test::test_sample_weighted ... ok
test distributions::weighted_index::test::test_weightedindex ... ok
test seq::index::test::value_stability_sample ... ok
test seq::index::test::test_sample_alg ... ok
test seq::test::test_multiple_weighted_edge_cases ... ok
test seq::test::test_partial_shuffle ... ok
test seq::test::test_sample_iter ... ok
test seq::test::test_iterator_choose ... ok
test seq::test::test_slice_choose ... ok
test distributions::bernoulli::test::test_average ... ok
test seq::test::value_stability_choose ... ok
test seq::test::value_stability_choose_multiple ... ok
test seq::test::value_stability_choose_stable ... ok
test seq::test::value_stability_slice ... ok
test test::test_random ... ok
test seq::test::test_shuffle ... ok
test seq::test::test_weighted ... ok
test rng::test::test_gen_ratio_average ... ok
test seq::test::test_multiple_weighted_distributions ... ok
test seq::test::test_iterator_choose_stable ... ok
test seq::test::test_iterator_choose_stable_stability ... ok
test distributions::uniform::tests::test_integers ... ok

test result: ok. 72 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
