
$ RUSTFLAGS="-C target-cpu=native" cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/memchr-perf-regression ./regress-nightly_2021-03-10-native-buildstd
