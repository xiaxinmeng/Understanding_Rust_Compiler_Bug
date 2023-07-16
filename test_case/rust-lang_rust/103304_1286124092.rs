
running 1 test
test test_measurement_time ... 
--- STDERR:              criterion::criterion_tests test_measurement_time ---
Benchmarking test_meas_time_1
Benchmarking test_meas_time_1: Warming up for 250.00 ms
AddressSanitizer:DEADLYSIGNAL
=================================================================
==16137==ERROR: AddressSanitizer: SEGV on unknown address 0x10009b671c3f (pc 0x5598a8f92815 bp 0x7fffdb3ce230 sp 0x7fffdb3ce130 T0)
==16137==The signal is caused by a READ memory access.
    #0 0x5598a8f92815 in core::ptr::read_volatile::h1e9c02ea583b8e0d (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x61c815) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #1 0x5598a90fc4dd in criterion::black_box::hae404db5835d6cdd (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x7864dd) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #2 0x5598a90d7555 in criterion::bencher::Bencher$LT$M$GT$::iter::h93fee6ddc6cc6ce0 (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x761555) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #3 0x5598a8f4740f in criterion_tests::test_measurement_time::_$u7b$$u7b$closure$u7d$$u7d$::hea2cd86757806f7a (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x5d140f) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #4 0x5598a8faf6cf in criterion::benchmark_group::BenchmarkGroup$LT$M$GT$::bench_function::_$u7b$$u7b$closure$u7d$$u7d$::hd042ce055f6fd19b (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x6396cf) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #5 0x5598a8df5d6b in _$LT$criterion..routine..Function$LT$M$C$F$C$T$GT$$u20$as$u20$criterion..routine..Routine$LT$M$C$T$GT$$GT$::warm_up::hf7b6b8ea8839b272 (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x47fd6b) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #6 0x5598a8e34587 in criterion::routine::Routine::sample::h833d2d40f718aa34 (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x4be587) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #7 0x5598a8f7d438 in criterion::analysis::common::h54916339714b3685 (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x607438) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #8 0x5598a8fd9d93 in criterion::benchmark_group::BenchmarkGroup$LT$M$GT$::run_bench::h6ffad90df37824ec (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x663d93) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #9 0x5598a8facdf2 in criterion::benchmark_group::BenchmarkGroup$LT$M$GT$::bench_function::hc758120eb1b398de (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x636df2) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #10 0x5598a90f5658 in criterion::Criterion$LT$M$GT$::bench_function::h43c1b741f0e071ea (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x77f658) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #11 0x5598a8f46b2b in criterion_tests::test_measurement_time::h87cadba36f25acc1 (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x5d0b2b) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
    #12 0x5598a8f46122 in criterion_tests::test_measurement_time::_$u7b$$u7b$closure$u7d$$u7d$::h1150f3d38a166b6d (/root/build/target/x86_64-unknown-linux-gnu/debug/deps/criterion_tests-94e045d31360a092+0x5d0122) (BuildId: 46d3bcc4e5b56eee8af8b7d71a998ef2f69c7df4)
