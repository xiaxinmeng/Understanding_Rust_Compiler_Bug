sh
cd /tmp
git clone https://github.com/rust-fuzz/honggfuzz-rs.git
cd honggfuzz-rs/example/
RUSTFLAGS="-Z sanitizer=thread" ./test.sh
