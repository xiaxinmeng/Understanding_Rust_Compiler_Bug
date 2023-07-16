sh
# Compile with instrumentation
rustc -Cprofile-generate=target/pgo src/main.rs
# Run the instrumented binary
target/release/proggy
# Run llvm-profdata to bring profiling data into usable state
llvm-profdata merge -output=target/pgo/proggy.profdata target/pgo
# Run the compiler again
rustc -Cprofile-use=target/pgo/proggy.profdata src/main.rs
