
> RUSTFLAGS="--remap-path-prefix=/home/joshua/src/rust/test-rustdoc=src --remap-path-prefix=/home/joshua/.local/lib/cargo=cargo --remap-path-prefix=/home/joshua/.local/lib/rustup=rustup" cargo run --release
   Compiling engine v0.1.0 (/home/joshua/src/rust/test-rustdoc/engine)
   Compiling hello-world v0.1.0 (/home/joshua/src/rust/test-rustdoc/hello-world)
    Finished release [optimized] target(s) in 0.52s
     Running `/home/joshua/.local/lib/cargo/target/release/hello-world`
$ strings /home/joshua/.local/lib/cargo/target/release/hello-world | grep home
home_dir
_ZN3std3sys4unix2os8home_dir17h5d3b091b9e7ed062E
home_dir
_ZN3std3sys4unix2os8home_dir8fallback17hb3ac94fa9a85ddbaE
_ZN3std3sys4unix2os8home_dir28_$u7b$$u7b$closure$u7d$$u7d$17ha072306bde63d38dE
_ZN3std3env8home_dir17h055eda4c1c3735acE
