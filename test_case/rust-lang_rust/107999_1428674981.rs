
RUSTFLAGS="-Zunpretty=mir -Zmir-enable-passes=+InstCombine -Zinline-mir" cargo +nightly run
