sh
$ rustc --test -Copt-level=3 -Ctarget-cpu=native 1.rs

$ ./1 --bench

running 5 tests
test bench_baseline                    ... bench:      65,459 ns/iter (+/- 118,871)
test bench_start_with_ascii_as_bytes   ... bench:     224,761 ns/iter (+/- 39,764)
test bench_start_with_ascii_char       ... bench:     450,245 ns/iter (+/- 66,865)
test bench_start_with_ascii_single_str ... bench:     452,953 ns/iter (+/- 61,812)
test bench_start_with_literal_char     ... bench:     221,022 ns/iter (+/- 33,137)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured


$ rustc -vV
rustc 1.19.0-nightly (386b0b9d3 2017-05-14)
binary: rustc
commit-hash: 386b0b9d39274701f30d31ee6ce31c363c6036ea
commit-date: 2017-05-14
host: x86_64-apple-darwin
release: 1.19.0-nightly
LLVM version: 4.0

