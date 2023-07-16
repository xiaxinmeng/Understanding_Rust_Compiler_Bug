
git clone https://github.com/hicommonwealth/substrate.git
cd substrate
git checkout 00a400f82539e2f78e8ddbcd98aea512c87c5f3c
cd ..
mkdir smaller_repo
cd smaller_repo
mkdir primitives
cp ../substrate/Cargo.lock .
cp -r ../substrate/primitives/arithmetic/ ./primitives/ 
cp -r ../substrate/primitives/debug-derive/ ./primitives/ 
cp -r ../substrate/primitives/std/ ./primitives/ 
cd primitives/arithmetic
cargo +nightly build --release --target=wasm32-unknown-unknown
