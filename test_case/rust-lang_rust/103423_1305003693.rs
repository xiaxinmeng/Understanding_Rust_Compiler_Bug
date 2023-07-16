
rustup update nightly # so that we can use latest version of cargo
./x.py build --stage 1
rustup toolchain link stage1 build/aarch64-apple-darwin/stage1
CARGO_TARGET_DIR=target/stage1 cargo +stage1 build -p risingwave_stream
