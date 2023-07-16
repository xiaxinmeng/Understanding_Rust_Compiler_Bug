 Bash
$ git clone https://github.com/Geal/pgo-rust.git
$ cd pgo-rust
$ rustc +stage2 -Copt-level=3  -Ccodegen-units=1 -Cpgo-gen=test.profraw src/main.rs -o main-pgo-gen
$ ./main-pgo-gen 1000000000
$ ~/rust/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -output=pgo.profdata test.profraw
$ rustc +stage2 -C opt-level=3 -Ccodegen-units=1 -Cpgo-use=pgo.profdata src/main.rs -o main-pgo-use
$ rustc +stage2 -C opt-level=3 -Ccodegen-units=1 src/main.rs -o main
$ time ./main         1000000000
$ time ./main-pgo-use 1000000000
