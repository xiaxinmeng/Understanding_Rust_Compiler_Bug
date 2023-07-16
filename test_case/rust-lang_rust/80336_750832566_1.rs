
git clone https://github.com/paritytech/substrate.git
cd substrate/bin/node-template/node
export RUSTFLAGS="-Z incremental-info=yes -Z incremental-verify-ich=yes"
cargo build
// Make a change to src/lib.rs
cargo build
