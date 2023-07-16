console
$ diff -r bench-bad/ bench-good/
$ rustup override list | grep bench-
~/bench-good                           rust-good
~/bench-bad                            rust-bad
$ git -C ~/rust-good rev-parse HEAD
02c7b117dadcfea4416f53b4bd99828bf750871f
$ git -C ~/rust-bad rev-parse HEAD
5f90947c2c27d5d4ae30ccc0e83ffc95a8597128
$ (~/bench-good) cargo bench 
   Compiling libc v0.2.20
   Compiling rand v0.3.15
   Compiling bench-hashmap v0.1.0 (file://~/bench-good)
    Finished release [optimized] target(s) in 7.3 secs
     Running target/release/deps/bench_hashmap-0ee5f7a4b83ca3d0

running 1 test
test hashmap_insert ... bench:          39 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

$ (~/bench-good) cargo clean; env CARGO_INCREMENTAL=1 cargo bench
   Compiling libc v0.2.20
   Compiling rand v0.3.15
   Compiling bench-hashmap v0.1.0 (file://~/bench-good)
    Finished release [optimized] target(s) in 7.39 secs
     Running target/release/deps/bench_hashmap-0ee5f7a4b83ca3d0

running 1 test
test hashmap_insert ... bench:          38 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

$ (~/bench-bad) cargo bench
   Compiling libc v0.2.20
   Compiling rand v0.3.15
   Compiling bench-hashmap v0.1.0 (file://~/bench-bad)
    Finished release [optimized] target(s) in 7.27 secs
     Running target/release/deps/bench_hashmap-e5736083e8c2908f

running 1 test
test hashmap_insert ... bench:          38 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

$ (~/bench-bad) cargo clean; env CARGO_INCREMENTAL=1 cargo bench
   Compiling libc v0.2.20
   Compiling rand v0.3.15
   Compiling bench-hashmap v0.1.0 (file://~/bench-bad)
    Finished release [optimized] target(s) in 7.42 secs
     Running target/release/deps/bench_hashmap-e5736083e8c2908f

running 1 test
test hashmap_insert ... bench:         133 ns/iter (+/- 13)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured
