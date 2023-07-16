sh
$ RUSTFLAGS="-Zsanitizer=address" cargo run # error
$ RUSTFLAGS="-Zsanitizer=address" cargo run --target x86_64-apple-darwin # ok
