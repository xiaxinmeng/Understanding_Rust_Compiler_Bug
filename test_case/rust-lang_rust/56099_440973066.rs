bash
rustup default beta-2018-11-17
cargo new issue56099
cd issue56099
echo "pub (in ::xxx) struct Xxx {};" > src/main.rs
cargo run
