bash
$ rustc +nightly --version
rustc 1.22.0-nightly (368122087 2017-09-06)

$ RUSTFLAGS="-Zremap-path-prefix-from=/Users -Zremap-path-prefix-to=/Hello" cargo +nightly build
   Compiling piston-texture v0.5.0
   Compiling byteorder v1.0.0
   Compiling either v1.1.0
   Compiling core-foundation-sys v0.4.1
   Compiling odds v0.2.25
   Compiling lzw v0.10.0
   Compiling current v0.1.2
   Compiling unicode-xid v0.0.4
error[E0583]: file not found for module `tables`
  --> /Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.0.4/src/lib.rs:57:5
   |
57 | mod tables;
   |     ^^^^^^
   |
   = help: name the file either tables.rs or tables/mod.rs inside the directory "/Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.0.4/src"

error: aborting due to previous error
