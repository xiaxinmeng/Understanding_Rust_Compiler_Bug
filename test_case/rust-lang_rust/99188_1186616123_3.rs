
$ cargo +stage1 clean -p fp-http && time cargo +stage1 build -p fp-http
   Compiling fp-http v0.1.0 (/home/amos/work/fly-proxy/crates/fp-http)
    Finished dev [unoptimized + debuginfo] target(s) in 46.68s
cargo +stage1 build -p fp-http  50.31s user 0.85s system 109% cpu 46.694 total
