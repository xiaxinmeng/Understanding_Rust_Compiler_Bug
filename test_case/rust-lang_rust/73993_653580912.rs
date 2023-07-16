sh
git clone git@github.com:hicommonwealth/edgeware-node.git
cd edgeware-node
git checkout 893981a2f2fdff8b3e6b0dddf4cf7ad4f9a09dfe
rustup override set nightly-2020-06-28
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-06-28-x86_64-unknown-linux-gnu
cargo build --release -p edgeware-runtime
