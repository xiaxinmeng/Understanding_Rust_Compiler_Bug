
$ cargo +beta --version
cargo 1.37.0-beta (4c1fa54d1 2019-06-24)
$ rustc +beta --version
rustc 1.37.0-beta.1 (178aa6611 2019-07-04)
$ cargo +beta new t1
     Created binary (application) `t1` package
$ cp -r t1 t2
$ cd t1
$ RUSTFLAGS="--remap-path-prefix=$PWD=." CARGO_INCREMENTAL=0 cargo +beta rustc --release -- -C codegen-units=1
   Compiling t1 v0.1.0 (/tmp/t1)
    Finished release [optimized] target(s) in 0.32s
$ cd ../t2
$ RUSTFLAGS="--remap-path-prefix=$PWD=." CARGO_INCREMENTAL=0 cargo +beta rustc --release -- -C codegen-units=1
   Compiling t1 v0.1.0 (/tmp/t2)
    Finished release [optimized] target(s) in 0.28s
$ cd ../
$ diffoscope t[12]/target/release/t1
