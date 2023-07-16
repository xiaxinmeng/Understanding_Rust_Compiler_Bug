sh
cargo new foo
cd foo
cargo b --timings
grep ncpu target/cargo-timings/cargo-timing.html
