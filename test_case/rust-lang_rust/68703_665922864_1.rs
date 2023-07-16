bash
cargo clean
CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Copt-level=0 -Zpanic_abort_tests -Cpanic=abort" cargo b
CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Copt-level=0 -Zpanic_abort_tests -Cpanic=abort" cargo t
grcov target/debug/ --ignore "/*" -s . --llvm --branch --ignore-not-existing -o html -t html
