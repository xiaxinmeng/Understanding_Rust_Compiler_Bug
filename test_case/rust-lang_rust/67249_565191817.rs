
$ rustc --version
rustc 1.41.0-nightly (27d6f55f4 2019-12-11)
$ rustc -C opt-level=3 --test starts_with.rs
$ ./starts_with --bench

running 4 tests
test bench_ends_with_char     ... bench:         437 ns/iter (+/- 21)
test bench_ends_with_string   ... bench:       1,405 ns/iter (+/- 86)
test bench_starts_with_char   ... bench:         713 ns/iter (+/- 19)
test bench_starts_with_string ... bench:         995 ns/iter (+/- 37)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out

$ ./rust/build/x86_64-apple-darwin/stage2/bin/rustc --version
rustc 1.41.0-dev
$ ./rust/build/x86_64-apple-darwin/stage2/bin/rustc -C opt-level=3 --test starts_with.rs
$ ./starts_with --bench

running 4 tests
test bench_ends_with_char     ... bench:         395 ns/iter (+/- 13)
test bench_ends_with_string   ... bench:         314 ns/iter (+/- 29)
test bench_starts_with_char   ... bench:         315 ns/iter (+/- 12)
test bench_starts_with_string ... bench:         315 ns/iter (+/- 24)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
