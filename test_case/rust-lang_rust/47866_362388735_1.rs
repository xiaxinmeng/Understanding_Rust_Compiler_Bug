
+ rustc -vV
rustc 1.24.0-nightly (77efd6800 2017-12-15)
binary: rustc
commit-hash: 77efd6800c57ba83923dddbbabf03c7afa6a34a4
commit-date: 2017-12-15
host: x86_64-unknown-linux-gnu
release: 1.24.0-nightly
LLVM version: 4.0
+ rustc foo.rs -O -C codegen-units=1
+ perf stat -e cycles ./foo

 Performance counter stats for './foo':

     3,129,021,965      cycles

       0.806494586 seconds time elapsed
