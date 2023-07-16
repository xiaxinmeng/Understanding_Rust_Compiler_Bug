sh
apk add build-base curl curl-dev pkgconf
curl https://sh.rustup.rs -sSf | sh -s -- -y
. ~/.cargo/env
cargo init --bin tester
cd tester
cargo add curl
echo 'use curl::easy::Easy; fn main() { Easy::new(); }' > src/main.rs
cargo build
