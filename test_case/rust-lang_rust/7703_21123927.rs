
~ > env RUST_MIN_STACK=100000000 ./test --bench

running 4 tests
test bench_bitv_big_each ... bench: 3731 ns/iter (+/- 1458)
test bench_bitv_small_each ... bench: 229 ns/iter (+/- 126)
test bench_btv_big_iter ... bench: 3754 ns/iter (+/- 1850)
test bench_btv_small_iter ... bench: 248 ns/iter (+/- 135)
