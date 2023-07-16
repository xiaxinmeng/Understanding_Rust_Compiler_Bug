
+ rustc -vV
rustc 1.24.0-nightly (50f6c3ece 2017-12-15)
binary: rustc
commit-hash: 50f6c3ece0ec738da48f8e77e6379a14bd02d1f4
commit-date: 2017-12-15
host: x86_64-unknown-linux-gnu
release: 1.24.0-nightly
LLVM version: 4.0
+ rustc foo.rs -O -C codegen-units=1
+ perf stat -e cycles ./foo

 Performance counter stats for './foo':

       331,404,612      cycles

       0.085892079 seconds time elapsed
