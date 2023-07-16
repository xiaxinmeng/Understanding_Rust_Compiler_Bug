
$ rustc -Vv
rustc 1.64.0-nightly (263edd43c 2022-07-17)
binary: rustc
commit-hash: 263edd43c5255084292329423c61a9d69715ebfa
commit-date: 2022-07-17
host: x86_64-unknown-linux-gnu
release: 1.64.0-nightly
LLVM version: 14.0.6

$ cargo bench --bench set_sum
[...]
running 6 tests
test hashbrown_set_sum_parallel  ... bench:     214,551 ns/iter (+/- 5,456)
test hashbrown_set_sum_serial    ... bench:   1,828,014 ns/iter (+/- 14,492)
test rayon_hash_set_sum_parallel ... bench:     464,519 ns/iter (+/- 53,670)
test rayon_hash_set_sum_serial   ... bench:   6,327,378 ns/iter (+/- 57,630)
test std_hash_set_sum_parallel   ... bench:   4,392,456 ns/iter (+/- 1,768,937)
test std_hash_set_sum_serial     ... bench:   2,904,370 ns/iter (+/- 18,217)

$ RUSTFLAGS=-Ccodegen-units=1 cargo bench --bench set_sum
[...]
running 6 tests
test hashbrown_set_sum_parallel  ... bench:     217,752 ns/iter (+/- 43,351)
test hashbrown_set_sum_serial    ... bench:   1,624,025 ns/iter (+/- 23,969)
test rayon_hash_set_sum_parallel ... bench:     494,929 ns/iter (+/- 50,196)
test rayon_hash_set_sum_serial   ... bench:   4,843,448 ns/iter (+/- 72,878)
test std_hash_set_sum_parallel   ... bench:   2,632,228 ns/iter (+/- 628,283)
test std_hash_set_sum_serial     ... bench:   1,773,990 ns/iter (+/- 66,189)
