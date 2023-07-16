
cargo install cargo-watch
cargo install wasm-pack

git clone -b wip/wd/rust-bug-68324 https://github.com/enso-org/ide.git

cd ide
./script/watch.sh
