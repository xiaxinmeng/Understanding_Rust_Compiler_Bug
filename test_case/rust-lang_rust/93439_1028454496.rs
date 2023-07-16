sh
$ RUSTFLAGS="-Z cf-protection=full" RUSTC="rustc-custom" cargo +nightly build -Z build-std --target x86_64-unknown-linux-gnu
...

$ readelf -a target/x86_64-unknown-linux-gnu/debug/empty | grep feature:
      Properties: x86 feature: IBT, SHSTK
