
$ rustc +nightly foo.rs -C link-args=-zstack-size=1024 -O -C lto --target wasm32-unknown-unknown && wasm-gc foo.wasm && wasm2wat foo.wasm
(module
  (type (;0;) (func (result i32)))
  (func $foo (type 0) (result i32)
    i32.const 1024)
  (table (;0;) 1 1 anyfunc)
  (memory (;0;) 1)
  (global (;0;) i32 (i32.const 1027))
  (global (;1;) i32 (i32.const 1027))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "__heap_base" (global 0))
  (export "__data_end" (global 1))
  (export "foo" (func $foo))
  (data (i32.const 1024) "foo"))
