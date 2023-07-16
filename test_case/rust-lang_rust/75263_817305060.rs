
$ RUSTFLAGS=--remap-path-prefix=/home/joshua/test-rustdoc/hello-world=src' --remap-path-prefix=/home/joshua/.local/lib/cargo=cargo --remap-path-prefix=/home/joshua/.local/lib/rustup=rustup' cargo run --release
$ strings target/release/hello-world | grep joshua | wc -l
0
