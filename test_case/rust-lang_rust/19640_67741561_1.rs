
rustc 3dcd21574 2014-11-17 (Servo snapshot)
test bench::push_str_one_byte            ... bench:    558857 ns/iter (+/- 44549) = 178 MB/s

rustc 99d6956c3 2014-12-18 (Nightly binary)
test bench::push_str_one_byte            ... bench:     85366 ns/iter (+/- 4261) = 1171 MB/s

rustc 8f51ad242 2014-12-20 (make check-stage1-collections PLEASE_BENCH=1)
test string::tests::bench_push_str_one_byte                ... bench:     39258 ns/iter (+/- 10864) = 254 MB/s
