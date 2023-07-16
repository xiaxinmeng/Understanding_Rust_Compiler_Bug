bash
cargo new rustc_driver_test

cd rustc_driver_test

echo "nightly-2020-06-16\n" > ./rust-toolchain

printf "\
#![feature(rustc_private)]\n\
extern crate rustc_driver;\n\
fn main() {}\n\
" > ./src/main.rs

rustup component add rust-src rustc-dev llvm-tools-preview

cargo +nightly build
