rust
~/M/benchmark-bytes-test ❯❯❯ cargo +stage1 bench                                                                                                                                                                                             ✘ 130 
   Compiling benchmark-bytes-test v0.1.0 (/home/jamie/Misc/benchmark-bytes-test)
    Finished release [optimized] target(s) in 0.37s
     Running target/release/deps/benchmark_bytes_test-80ad362b39f8674c

running 5 tests
test tests::bench_50_b  ... bench:  20,217,836 ns/iter (+/- 260,903) = 49 B/s
test tests::bench_50_gb ... bench:     151,659 ns/iter (+/- 2,454) = 39 GB/s
test tests::bench_50_kb ... bench:     151,641 ns/iter (+/- 1,520) = 39 kB/s
test tests::bench_50_mb ... bench:     151,586 ns/iter (+/- 271) = 39 MB/s
test tests::bench_50_tb ... bench:     151,668 ns/iter (+/- 1,658) = 39560 GB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out
