sh
git clone https://github.com/BurntSushi/memchr
cd memchr
RUSTFLAGS="-Z sanitizer=thread" \
        RUSTDOCFLAGS="-Z sanitizer=thread" \
        RUST_TEST_THREADS=1 \
        cargo +nightly build
