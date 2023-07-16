
RUSTFLAGS="-Ctarget-feature=+multivalue -Cpanic=abort" cargo +nightly build --target wasm32-unknown-unknown --release -Zbuild-std=panic_abort,std
