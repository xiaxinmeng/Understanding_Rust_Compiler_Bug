
test bench_stream_1000_empty_lists   ... bench:      11,147 ns/iter (+/- 362)
test bench_stream_nested_empty_lists ... bench:         278 ns/iter (+/- 247)
test bench_stream_u256_value         ... bench:         609 ns/iter (+/- 40)
test bench_stream_u64_value          ... bench:         154 ns/iter (+/- 13)

time to run ~/.cargo/bin/rustup run nightly cargo bench -p ethcore-util --bench rlp -j1
99% (563.41 real, 14.16 kernel, 544.92 user); 454708k resident
