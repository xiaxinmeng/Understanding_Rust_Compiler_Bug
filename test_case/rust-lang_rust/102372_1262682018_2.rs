console
  $ RUSTFLAGS="-C target-feature=+atomics,+bulk-memory" ./x.py build library --target wasm32-wasi
  $ rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1
  