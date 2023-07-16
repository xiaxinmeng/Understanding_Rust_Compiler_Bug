sh
RUSTFLAGS="-C link-args=-zstack-size=48000" cargo +nightly-2018-11-27 build --release --target=wasm32-unknown-unknown -vv -p hello_bare
   Compiling hello_bare v0.1.0 (C:\Users\sagan\Documents\sagan-software\rust-eos\examples\hello_bare)
     Running `rustc --crate-name hello_bare 'examples\hello_bare\src\lib.rs' --color always --crate-type cdylib --emit=dep-info,link -C opt-level=s -C panic=abort -C lto -C metadata=673de3ef755438ae --out-dir 'C:\Users\sagan\Documents\sagan-software\rust-eos\target\wasm32-unknown-unknown\release\deps' --target wasm32-unknown-unknown -L 'dependency=C:\Users\sagan\Documents\sagan-software\rust-eos\target\wasm32-unknown-unknown\release\deps' -L 'dependency=C:\Users\sagan\Documents\sagan-software\rust-eos\target\release\deps' -C link-args=-zstack-size=48000`
    Finished release [optimized] target(s) in 1.24s
wasm2wat target/wasm32-unknown-unknown/release/hello_bare.wasm -o target/wasm32-unknown-unknown/release/hello_bare.wat --generate-names
tail target/wasm32-unknown-unknown/release/hello_bare.wat
  (memory $memory 17)
  (global $g0 (mut i32) (i32.const 1048576))
  (global $__heap_base i32 (i32.const 1048580))
  (global $__data_end i32 (i32.const 1048580))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "apply" (func $apply))
  (data (i32.const 1048576) "Hi, "))
