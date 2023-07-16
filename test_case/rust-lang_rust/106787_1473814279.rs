sh
git clone https://github.com/dtolnay/clang-ast
cd clang-ast
git checkout 0.1.13
cargo b --release --all-features
env RUSTFLAGS='-C opt-level=3 -C embed-bitcode=no' cargo b --release --all-features
