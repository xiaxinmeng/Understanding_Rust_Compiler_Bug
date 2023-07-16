
$ clang++ -O2 -fPIC -std=c++11 -c foo.cpp
$ ar rc libfoo.a foo.o
$ LD_LIBRARY_PATH=x86_64-unknown-linux-gnu/stage1/lib ./x86_64-unknown-linux-gnu/stage1/bin/rustc --test -O -L native=. foo.rs
$ ./foo --bench

running 2 tests
test cpp ... bench:       130 ns/iter (+/- 9)
test x   ... bench:        83 ns/iter (+/- 5)
