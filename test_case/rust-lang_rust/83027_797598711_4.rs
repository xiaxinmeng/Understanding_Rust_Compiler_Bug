
$ cargo +beta build --release && cp target/release/memchr-perf-regression ./regress-beta
$ RUSTFLAGS="-C target-cpu=native" cargo +beta build --release && cp target/release/memchr-perf-regression ./regress-beta-native
