
$ rustc -Vv
rustc 1.25.0-nightly (4d2d3fc5d 2018-02-13)
binary: rustc
commit-hash: 4d2d3fc5dadf894a8ad709a5860a549f2c0b1032
commit-date: 2018-02-13
host: x86_64-unknown-linux-gnu
release: 1.25.0-nightly
LLVM version: 6.0

$ cargo bench --bench set_sum
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling [...]
    Finished release [optimized] target(s) in 5.37 secs
     Running target/release/deps/set_sum-d85cde9ece817246

running 4 tests
test rayon_set_sum_parallel ... bench:   1,420,138 ns/iter (+/- 34,709)
test rayon_set_sum_serial   ... bench:   7,718,603 ns/iter (+/- 141,127)
test std_set_sum_parallel   ... bench:   8,886,208 ns/iter (+/- 137,102)
test std_set_sum_serial     ... bench:   7,845,670 ns/iter (+/- 106,685)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out

$ RUSTFLAGS=-Ccodegen-units=1 cargo bench --bench set_sum
   Compiling [...]
    Finished release [optimized] target(s) in 6.40 secs
     Running target/release/deps/set_sum-d85cde9ece817246

running 4 tests
test rayon_set_sum_parallel ... bench:   1,089,784 ns/iter (+/- 167,357)
test rayon_set_sum_serial   ... bench:   6,196,760 ns/iter (+/- 335,661)
test std_set_sum_parallel   ... bench:   8,497,259 ns/iter (+/- 128,929)
test std_set_sum_serial     ... bench:   6,151,935 ns/iter (+/- 76,954)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
