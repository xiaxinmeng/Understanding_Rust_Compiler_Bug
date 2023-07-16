
docker run -it ubuntu:18.04 bash
apt update
apt install curl git gcc make musl-tools file
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add x86_64-unknown-linux-musl
git clone https://github.com/mozilla/sccache.git
cd sccache
export TARGET=x86_64-unknown-linux-musl && export OPENSSL_DIR=/openssl-musl
./scripts/travis-musl-openssl.sh
cargo build --target x86_64-unknown-linux-musl
RUST_LOG=sccache=debug RUST_BACKTRACE=1 SCCACHE_NO_DAEMON=1 SCCACHE_START_SERVER=1 $(pwd)/target/x86_64-unknown-linux-musl/debug/sccache &
RUST_LOG=sccache=debug $(pwd)/target/x86_64-unknown-linux-musl/debug/sccache gcc -c src/test/test.c -o /tmp/test.o
RUST_LOG=sccache=debug $(pwd)/target/x86_64-unknown-linux-musl/debug/sccache gcc -c src/test/test.c -o /tmp/test.o
