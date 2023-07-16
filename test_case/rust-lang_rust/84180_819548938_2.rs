sh
#!/bin/sh -ex

env RUSTFLAGS="-Zinstrument-coverage" cargo +nightly test
llvm-profdata merge -sparse default.profraw -o default.profdata
llvm-cov show ./target/debug/deps/kangaroo-f715d112c2e2f815 -instr-profile=default.profdata --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/.rustup/toolchains/'
