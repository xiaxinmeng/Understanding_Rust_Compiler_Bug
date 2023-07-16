console
$ cargo +nightly bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/res_init_bench-94665f8bc2ebf0bc

running 3 tests
test tests::bench_plain  ... bench:   4,419,043 ns/iter (+/- 331,978)
test tests::bench_reinit ... bench:   4,420,291 ns/iter (+/- 297,022)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

$ sudo netctl stop-all
$ cargo +nightly bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/res_init_bench-94665f8bc2ebf0bc

running 3 tests
test tests::bench_plain  ... bench:     267,152 ns/iter (+/- 18,776)
test tests::bench_reinit ... bench:     267,891 ns/iter (+/- 9,855)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
