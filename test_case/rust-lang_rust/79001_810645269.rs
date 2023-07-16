
cargo install hyperfine
git clone https://github.com/rust-lang/rustc-perf/
cd rustc-perf/collector/benchmarks/packed-simd
hyperfine --prepare 'cargo clean' 'cargo +stage1 build'
