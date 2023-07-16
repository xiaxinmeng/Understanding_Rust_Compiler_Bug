
$ taskset 101 cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests (target/release/deps/tests-1a475deaa7086b68)

running 5 tests
test tests::it_works ... ignored
test tests::bench_default                 ... bench:  91,475,188 ns/iter (+/- 1,344,571)
test tests::bench_stable_binary_search_by ... bench:  83,976,120 ns/iter (+/- 1,237,765)
test tests::bench_stable_no_unwrap        ... bench:  77,808,973 ns/iter (+/- 568,975)
test tests::bench_stable_unwrap           ... bench:  86,669,423 ns/iter (+/- 921,592)

test result: ok. 0 passed; 0 failed; 1 ignored; 4 measured; 0 filtered out; finished in 102.33s
