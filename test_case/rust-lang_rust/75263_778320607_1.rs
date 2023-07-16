
$ cargo new hello-world
$ cd hello-world
$ echo '[profile.release]
panic = "abort"' >> Cargo.toml
$ RUSTFLAGS="--remap-path-prefix=/home/joshua/test-rustdoc/hello-world=src --remap-path-prefix=/home/joshua/.local/lib/cargo=cargo --remap-path-prefix=/home/joshua/.local/lib/rustup=rustup" cargo run --release
$ strings /home/joshua/.local/lib/cargo/target/release/hello-world | grep home
home_dir
_ZN3std3sys4unix2os8home_dir17h5d3b091b9e7ed062E
home_dir
_ZN3std3sys4unix2os8home_dir8fallback17hb3ac94fa9a85ddbaE
_ZN3std3sys4unix2os8home_dir28_$u7b$$u7b$closure$u7d$$u7d$17ha072306bde63d38dE
_ZN3std3env8home_dir17h055eda4c1c3735acE