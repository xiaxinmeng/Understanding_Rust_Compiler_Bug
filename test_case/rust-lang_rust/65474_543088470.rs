
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup toolchain install nightly-2019-10-16
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup target add --toolchain nightly-2019-10-16 x86_64-fortanix-unknown-sgx
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup target add --toolchain nightly-2019-10-16 x86_64-unknown-linux-musl
cargo new hw
cd hw
cargo +nightly-2019-10-16 build --target x86_64-unknown-linux-musl
cargo +nightly-2019-10-16 build --target x86_64-fortanix-unknown-sgx
