
RUSTFLAGS="-Zmir-enable-passes=+InstCombine -Zinline-mir" cargo +nightly build
