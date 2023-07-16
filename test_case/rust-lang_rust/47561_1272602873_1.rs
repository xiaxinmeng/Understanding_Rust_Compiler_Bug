
RUSTFLAGS="-Ccodegen-units=1 -Cllvm-args=-inline-threshold=100000" cargo bench
