
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
    i32.const 128)
  (table (;0;) 1 1 anyfunc)
  (memory (;0;) 2)
  (export "memory" (memory 0))
  (export "foo" (func $foo))
  (data (i32.const 128) "foo"))
$ rustc +nightly foo.rs -C link-args=--global-base=1000 -O -C lto --target wasm32-unknown-unknown && wasm-gc foo.wasm && wasm2wat foo.wasm
(module
  (type (;0;) (func (result i32)))
  (func $foo (type 0) (result i32)
    i32.const 1000)
  (table (;0;) 1 1 anyfunc)
  (memory (;0;) 2)
  (export "memory" (memory 0))
  (export "foo" (func $foo))
  (data (i32.const 1000) "foo"))
