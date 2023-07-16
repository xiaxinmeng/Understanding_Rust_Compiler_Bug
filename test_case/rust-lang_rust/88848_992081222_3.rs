
$ taskset 101 cargo bench # swap the name
   Compiling tests v0.1.0 (/me/tests)
    Finished bench [optimized] target(s) in 10.40s
     Running unittests (target/release/deps/tests-1a475deaa7086b68)

running 5 tests
test tests::it_works ... ignored
test tests::bench_default                 ... bench:  99,601,964 ns/iter (+/- 1,446,100)
test tests::bench_stable_binary_search_by ... bench:  86,524,126 ns/iter (+/- 977,896)
test tests::bench_stable_no_unwrap        ... bench:  83,498,449 ns/iter (+/- 997,731)
test tests::bench_stable_unwrap           ... bench:  84,759,333 ns/iter (+/- 2,089,094)

test result: ok. 0 passed; 0 failed; 1 ignored; 4 measured; 0 filtered out; finished in 106.27s
