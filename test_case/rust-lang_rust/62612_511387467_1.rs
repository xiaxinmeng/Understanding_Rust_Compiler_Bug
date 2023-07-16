
~ rustup default nightly
~ rustup component add rust-src
~ rustc --edition=2018 --crate-name core ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/lib.rs --color always --crate-type lib --emit=link,llvm-bc -C opt-level=0 --out-dir /tmp/ --target mipsel-unknown-none
