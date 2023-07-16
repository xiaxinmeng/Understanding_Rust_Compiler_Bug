
Rust nightly-10-09
cargo +nightly-2021-10-09 build --lib --release;LD_AUDIT=/home/user/readenv/target/release/libreadenv.so /usr/bin/test || true
   Compiling readenv v0.1.0 (/home/user/readenv)
    Finished release [optimized] target(s) in 1.09s
Rust nightly-10-10
cargo +nightly-2021-10-10 build --lib --release;LD_AUDIT=/home/user/readenv/target/release/libreadenv.so /usr/bin/test || true
   Compiling readenv v0.1.0 (/home/user/readenv)
    Finished release [optimized] target(s) in 0.88s
Segmentation fault
