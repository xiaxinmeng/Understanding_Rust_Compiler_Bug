
mkdir musl-test
cd musl-test
cargo init
cargo install cross
cross build --target x86_64-unknown-linux-musl 
