
$ ls
Cargo.lock  Cargo.toml  src  target  test.sh

$ RUSTFLAGS="-C target-feature=+crt-static" cargo +nightly run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/issue-94564`
Cargo.lock  Cargo.toml	src  target  test.sh
