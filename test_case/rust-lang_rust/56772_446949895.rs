
rustc +nightly -C opt-level=0 lib.rs
rustc +nightly --extern lib=liblib.rlib -C opt-level=0 test.rs
./test
