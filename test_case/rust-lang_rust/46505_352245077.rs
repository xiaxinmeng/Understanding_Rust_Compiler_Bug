bash
ls /cores
cargo new --bin rust-foo && cd rust-foo
cargo build
ulimit -c unlimited
touch src/main.rs && time cargo +stable build # Doesn't generate a dump.
ls /cores
touch src/main.rs && time cargo +nightly build # Does generate a dump (thus slower).
ls /cores
ulimit -c 0
touch src/main.rs && time cargo +stable build
touch src/main.rs && time cargo +nightly build # Faster as dumps turned off.
cd .. && rm -rf rust-foo
