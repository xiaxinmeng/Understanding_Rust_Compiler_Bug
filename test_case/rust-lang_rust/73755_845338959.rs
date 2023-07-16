
$ cargo rustc --release --target wasm32-wasi -- -C target-feature=+multivalue
...
= note: rust-lld: error: function signature mismatch: _ZN3std3ffi5c_str7CString18from_vec_unchecked17heb55af31731c143eE
          >>> defined as (i32) -> i32 in /home/direnc/workspace/nodejs-rust/smartcore-wasi/target/wasm32-wasi/release/deps/smartcore_wasi_lib.smartcore_wasi_lib.bwxwt9he-cgu.7.rcgu.o
          >>> defined as (i32, i32) -> void in /home/direnc/.rustup/toolchains/1.52.1-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-wasi/lib/libstd-3ee14805eea1e96d.rlib(std-3ee14805eea1e96d.std.4z7x2hk1-cgu.0.rcgu.o)
          
          rust-lld: error: function signature mismatch: _ZN3std3ffi5c_str7CString10into_inner17h5643fc78239ca534E
          >>> defined as (i32, i32) -> i32 in /home/direnc/workspace/nodejs-rust/smartcore-wasi/target/wasm32-wasi/release/deps/smartcore_wasi_lib.smartcore_wasi_lib.bwxwt9he-cgu.7.rcgu.o
          >>> defined as (i32, i32, i32) -> void in /home/direnc/.rustup/toolchains/1.52.1-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-wasi/lib/libstd-3ee14805eea1e96d.rlib(std-3ee14805eea1e96d.std.4z7x2hk1-cgu.0.rcgu.o)
          
          rust-lld: error: function signature mismatch: _ZN7bincode2de4read11SliceReader3new17h0c9b273d280dc95dE
          >>> defined as (i32, i32) -> i32 in /home/direnc/workspace/nodejs-rust/smartcore-wasi/target/wasm32-wasi/release/deps/smartcore_wasi_lib.smartcore_wasi_lib.bwxwt9he-cgu.15.rcgu.o
          >>> defined as (i32, i32, i32) -> void in /home/direnc/workspace/nodejs-rust/smartcore-wasi/target/wasm32-wasi/release/deps/libbincode-f7c047cbbfd01ab7.rlib(bincode-f7c047cbbfd01ab7.bincode.4dc53osx-cgu.14.rcgu.o)
