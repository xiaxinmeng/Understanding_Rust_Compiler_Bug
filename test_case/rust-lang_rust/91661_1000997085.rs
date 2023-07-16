
cargo clean
RUSTFLAGS="-Z instrument-coverage" cargo build
RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="json5format-%m.profraw" cargo test --tests
cargo profdata -- merge -sparse json5format-*.profraw -o json5format.profdata

// Analyze test data
cargo cov -- report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=json5format.profdata --object target/debug/deps/cov_test-86f2fdf290095642

// Analyze test data
cargo cov -- show --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=json5format.profdata --object target/debug/deps/rand-b20edb5fd980bac1 --object target/debug/deps/cov_test-e20c2ec08166eaaf --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt | less -R
