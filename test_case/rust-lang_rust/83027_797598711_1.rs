
$ cargo clean && cargo +stable build --release && cp ./target/release/rg ./rg-stable
$ cargo clean && RUSTFLAGS="-C target-cpu=native" cargo +stable build --release && cp ./target/release/rg ./rg-stable-native
$ cargo clean && cargo +beta build --release && cp ./target/release/rg ./rg-beta
$ cargo clean && RUSTFLAGS="-C target-cpu=native" cargo +beta build --release && cp ./target/release/rg ./rg-beta-native
