Rust
curl https://sh.rustup.rs -sSf | sh
and the whole "Building a Cross Compiler" should be replaced by

rustup target add aarch64-apple-ios 
rustup target add armv7-apple-ios 
rustup target add armv7s-apple-ios 
rustup target add x86_64-apple-ios 
rustup target add i386-apple-ios
