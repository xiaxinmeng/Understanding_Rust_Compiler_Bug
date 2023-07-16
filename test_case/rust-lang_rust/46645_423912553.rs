
$ cat foo.rs
#![crate_type = "cdylib"]

#[no_mangle]
pub extern fn foo() -> *const u8 {
    "foo".as_ptr()
}
$ rustc +nightly foo.rs -C link-args=--global-base=128 -O -C lto --target wasm32-unknown-unknown && wasm-gc foo.wasm && wasm2wat foo.wasm
(module
  (type (;0;) (func (result i32)))
  (func $foo (type 0) (result i32)
    i32.const 1048576)
  (table (;0;) 1 1 anyfunc)
  (memory (;0;) 17)
  (global (;0;) i32 (i32.const 1048579))
  (global (;1;) i32 (i32.const 1048579))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "__heap_base" (global 0))
  (export "__data_end" (global 1))
  (export "foo" (func $foo))
  (data (i32.const 1048576) "foo"))
