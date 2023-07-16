bash
export RUSTFLAGS="-Zprofile"
cargo test -p $pkg --lib --all-features --profile=coverage -Z unstable-options
# and collect coverage metrics respectively
grcov ./target/coverage/ -s . -t lcov --llvm --branch --ignore-not-existing --ignore "cli*" --ignore "*test*"  --ignore "target/*"  --ignore "*migrations*" --excl-start '#\[cfg\(test\)\]' --excl-stop '^}' -o $coverage_report
