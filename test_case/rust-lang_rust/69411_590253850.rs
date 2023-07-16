
RUSTDOCFLAGS="--persist-doctests target/doctests -Z unstable-options" cargo test --doc -- --test-threads=1
