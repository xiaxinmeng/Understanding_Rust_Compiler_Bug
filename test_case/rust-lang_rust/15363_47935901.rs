
$ go build foo.go
$ time ./foo
./foo  0.04s user 0.00s system 101% cpu 0.038 total
$ time GOMAXPROCS=8 ./foo
GOMAXPROCS=8 ./foo  0.20s user 0.09s system 208% cpu 0.140 total


$ rustc -O foo.rs
$ time ./foo
./foo  2.45s user 1.51s system 261% cpu 1.517 total
$ time RUST_THREADS=1 ./foo
RUST_THREADS=1 ./foo  0.13s user 0.00s system 99% cpu 0.136 total
