
   Compiling zookeeper_derive v0.3.0 (file:///home/jwolfe/Rust/rust-zookeeper/zookeeper-derive)
     Running `rustc --crate-name zookeeper_derive zookeeper-derive/src/lib.rs --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=3d948e35b92c7695 -C extra-filename=-3d948e35b92c7695 --out-dir /home/jwolfe/Rust/rust-zookeeper/target/release/deps -L dependency=/home/jwolfe/Rust/rust-zookeeper/target/release/deps --extern syn=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libsyn-b00a444c8aa4acf3.rlib --extern quote=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libquote-7cd2e6e51ec0ccc5.rlib -Zverbose`
   Compiling zookeeper v0.3.0 (file:///home/jwolfe/Rust/rust-zookeeper)
     Running `rustc --crate-name zookeeper src/lib.rs --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=b5a475f1c342e7c6 -C extra-filename=-b5a475f1c342e7c6 --out-dir /home/jwolfe/Rust/rust-zookeeper/target/release/deps -L dependency=/home/jwolfe/Rust/rust-zookeeper/target/release/deps --extern snowflake=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libsnowflake-e9a06768725f9ce6.rlib --extern bytes=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libbytes-896568c95020390b.rlib --extern byteorder=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libbyteorder-549a4dbae79b4adf.rlib --extern zookeeper_derive=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libzookeeper_derive-3d948e35b92c7695.so --extern log=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/liblog-03d566855f19b793.rlib --extern mio=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/libmio-c1cb259295b5b695.rlib --extern lazy_static=/home/jwolfe/Rust/rust-zookeeper/target/release/deps/liblazy_static-a9194fff651d7463.rlib -Zverbose`
error: /home/jwolfe/Rust/rust-zookeeper/target/release/deps/libzookeeper_derive-3d948e35b92c7695.so: undefined symbol: _ZN116_$LT$alloc..string..String$u20$as$u20$core..convert..From$LT$$RF$ReEarlyBound$LP$0$C$$u20$$u27$a$RP$$u20$str$GT$$GT$4from17h3156f71d21fd9c11E
  --> src/lib.rs:11:1
   |
11 | extern crate zookeeper_derive;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `zookeeper`.
