
$ rustc +nightly --crate-type staticlib -C debuginfo=1 -C codegen-units=1 -C opt-level=3 -C panic=abort -C target-cpu=native -C remark=loop-vectorize 1.rs

note: optimization analysis for loop-vectorize at 1.rs:9:0: loop not vectorized: loop control flow is not understood by vectorizer

note: optimization missed for loop-vectorize at 1.rs:9:0: loop not vectorized
