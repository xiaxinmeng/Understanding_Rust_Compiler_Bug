
neutron@Neutron:/me/rust/bench$ cargo bench
   Compiling bench v0.1.0 (/me/rust/bench)
    Finished bench [optimized] target(s) in 1.55s
     Running target/release/deps/bench-25133398f204ef70

running 6 tests
test tests::new_xpow0  ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow1  ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow3  ... bench:           2 ns/iter (+/- 1)
test tests::rust_xpow0 ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow1 ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow3 ... bench:           2 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out

neutron@Neutron:/me/rust/bench$ cargo bench
   Compiling bench v0.1.0 (/me/rust/bench)
    Finished bench [optimized] target(s) in 0.66s
     Running target/release/deps/bench-25133398f204ef70

running 8 tests
test tests::new_xpow0     ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow1     ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow3     ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow3123  ... bench:           9 ns/iter (+/- 2)
test tests::rust_xpow0    ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow1    ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow3    ... bench:           2 ns/iter (+/- 0)
test tests::rust_xpow3123 ... bench:           9 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out

neutron@Neutron:/me/rust/bench$ cargo bench
   Compiling bench v0.1.0 (/me/rust/bench)
    Finished bench [optimized] target(s) in 0.80s
     Running target/release/deps/bench-25133398f204ef70

running 10 tests
test tests::new_xpow0     ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow1     ... bench:           0 ns/iter (+/- 0)
test tests::new_xpow3     ... bench:           2 ns/iter (+/- 1)
test tests::new_xpow3123  ... bench:          10 ns/iter (+/- 3)
test tests::new_xpow7     ... bench:           2 ns/iter (+/- 0)
test tests::rust_xpow0    ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow1    ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow3    ... bench:           2 ns/iter (+/- 0)
test tests::rust_xpow3123 ... bench:          10 ns/iter (+/- 2)
test tests::rust_xpow7    ... bench:           3 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out

neutron@Neutron:/me/rust/bench$ cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/bench-25133398f204ef70

running 10 tests
test tests::new_xpow0     ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow1     ... bench:           1 ns/iter (+/- 0)
test tests::new_xpow3     ... bench:           2 ns/iter (+/- 1)
test tests::new_xpow3123  ... bench:           9 ns/iter (+/- 2)
test tests::new_xpow7     ... bench:           2 ns/iter (+/- 0)
test tests::rust_xpow0    ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow1    ... bench:           1 ns/iter (+/- 0)
test tests::rust_xpow3    ... bench:           2 ns/iter (+/- 0)
test tests::rust_xpow3123 ... bench:          10 ns/iter (+/- 4)
test tests::rust_xpow7    ... bench:           3 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured; 0 filtered out
