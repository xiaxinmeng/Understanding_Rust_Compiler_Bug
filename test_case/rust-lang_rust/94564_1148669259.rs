
$ rustc +nightly  --version
rustc 1.63.0-nightly (50b00252a 2022-06-06)
$ rustc +nightly 94564.rs -C opt-level=2 -C lto -C target-feature=+crt-static && ./94564
