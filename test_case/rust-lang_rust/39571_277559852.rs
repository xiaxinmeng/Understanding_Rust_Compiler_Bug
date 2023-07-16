
export RUST_STABLE_VERSION=`curl https://static.rust-lang.org/dist/channel-rust-stable | head -n 1 | cut -d"-" -f2`
export RUST_ARCHIVE=rust-$RUST_STABLE_VERSION-x86_64-unknown-linux-gnu.tar.gz
export RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE
