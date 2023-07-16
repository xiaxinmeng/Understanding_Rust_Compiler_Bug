
$ RUSTFLAGS="-C target-cpu=broadwell" cargo +nightly build --release && cp target/release/memchr-perf-regression ./regress-nightly_2021-03-10-broadwell
   Compiling memchr v2.3.4
   Compiling memchr-perf-regression v0.1.0 (/tmp/memchr-perf-regression)
    Finished release [optimized] target(s) in 1.11s

$ time ./regress-nightly_2021-03-10-broadwell

real    0.767
user    0.763
sys     0.003
maxmem  7 MB
faults  0
