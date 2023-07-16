
cargo install rustup-toolchain-install-master
rustup-toolchain-install-master -t wasm32-unknown-unknown -i x86_64-pc-windows-msvc c4ca2fa230dcaef515acf148ec70c1850bbe5dc1 # and/or i686-pc-windows-msvc
cargo +c4ca2fa230dcaef515acf148ec70c1850bbe5dc1 build # or however you reproduce
