bash
RUSTFLAGS="-C link-arg=-zstack-size=32768 -C target-feature=-mutable-globals,-sign-ext,-multivalue,-simd128" cargo build -Z build-std=core,alloc --release --target=wasm32-unknown-unknown
