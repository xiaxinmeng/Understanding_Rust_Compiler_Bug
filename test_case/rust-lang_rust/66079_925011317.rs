sh
git clone https://github.com/trailofbits/dylint -b register_tool
cd dylint
cargo +nightly build -p dylint
grep 'register_tool' dylint/src/lib.rs
