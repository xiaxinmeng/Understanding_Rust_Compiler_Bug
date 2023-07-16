
LLVM_PROFILE_FILE="your_name-%p-%m.profraw" cargo test --verbose
grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info
