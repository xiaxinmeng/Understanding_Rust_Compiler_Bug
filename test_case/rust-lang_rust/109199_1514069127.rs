sh
rustup default nightly-2023-03-21-x86_64-unknown-linux-gnu

git clone https://github.com/identity-com/on-chain-identity-gateway.git
cd on-chain-identity-gateway/solana
git checkout c939b6feb8aa92d596306a1aeb2dc497c2f7f693

cargo tarpaulin # it fails here at `sol-did` compilation
cargo build # it does not fail here, however, `sol-did` is compiled
