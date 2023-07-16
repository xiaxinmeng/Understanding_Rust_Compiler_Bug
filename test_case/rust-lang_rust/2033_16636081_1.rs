
landin:~ lkuper$ export RUST_LOG=debug=4; ./debug
rust: ~"error"
rust: ~"warn"
rust: ~"info"
rust: ~"debug"
landin:~ lkuper$ export RUST_LOG=debug=3; ./debug
rust: ~"error"
rust: ~"warn"
rust: ~"info"
landin:~ lkuper$ export RUST_LOG=debug=2; ./debug
rust: ~"error"
rust: ~"warn"
landin:~ lkuper$ export RUST_LOG=debug=1; ./debug
rust: ~"error"
landin:~ lkuper$ export RUST_LOG=debug=0; ./debug
landin:~ lkuper$
