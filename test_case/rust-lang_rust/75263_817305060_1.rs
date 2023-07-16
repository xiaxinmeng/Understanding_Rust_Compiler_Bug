
$ CARGO_INCREMENTAL=0 RUSTFLAGS=--remap-path-prefix=/home/joshua/test-rustdoc/hello-world=src' --remap-path-prefix=/home/joshua/.local/lib/cargo=cargo --remap-path-prefix=/home/joshua/.local/lib/rustup=rustup' cargo run -q
Hello, world!
$ strings target/debug/hello-world | grep joshua | wc -l
8
