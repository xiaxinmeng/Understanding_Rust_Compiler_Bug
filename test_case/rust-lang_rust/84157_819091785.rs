
brew install gcc

# This works
cargo clean && env -i PATH="$HOME/.cargo/bin:/usr/bin:/bin" cargo test

# This fails
cargo clean && env -i PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin" cargo test
