
test bench_stream_1000_empty_lists   ... bench:      26,937 ns/iter (+/- 3,985)
test bench_stream_nested_empty_lists ... bench:         842 ns/iter (+/- 131)
test bench_stream_u256_value         ... bench:       1,405 ns/iter (+/- 54)
test bench_stream_u64_value          ... bench:         582 ns/iter (+/- 19)

time to run ~/.cargo/bin/rustup run nightly cargo bench -p ethcore-util --bench rlp -j1
99% (600.02 real, 13.73 kernel, 581.89 user); 468572k resident
