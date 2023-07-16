sh
$ rustc +nightly -C codegen-units=1 -C opt-level=3 -C target-cpu=native 1.rs

$ time ./1 30000 2
13500450000000

real	0m5.389s
user	0m5.136s
sys	0m0.012s

$ time ./1 30000 1
13500450000000

real	0m2.195s
user	0m2.192s
sys	0m0.000s

$ rustc +38bd38147d2fa21f8a684b019fc0763adf8fd436 -C codegen-units=1 -C opt-level=3 -C target-cpu=native 1.rs

$ time ./1 30000 2
13500450000000

real	0m4.445s
user	0m4.332s
sys	0m0.000s

$ time ./1 30000 1
13500450000000

real	0m2.330s
user	0m2.184s
sys	0m0.016s
