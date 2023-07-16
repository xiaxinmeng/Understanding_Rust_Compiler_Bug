
rustup toolchain install 1.24.0-arm-unknown-linux-gnueabihf
mv ~/.rustup/toolchains/{1.24.0,stable}-arm-unknown-linux-gnueabihf
mv ~/.rustup/update-hashes/{1.24.0,stable}-arm-unknown-linux-gnueabihf
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup update stable-arm-unknown-linux-gnueabihf
