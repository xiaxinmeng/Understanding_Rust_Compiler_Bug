
$ cargo build
   Compiling proc-macro2 v0.4.24
   Compiling unicode-xid v0.1.0
   Compiling wasm-bindgen-shared v0.2.29
   Compiling cfg-if v0.1.6
   Compiling lazy_static v1.2.0
   Compiling wasm-bindgen v0.2.29
   Compiling log v0.4.6
   Compiling quote v0.6.10
   Compiling syn v0.15.23
   Compiling wasm-bindgen-backend v0.2.29
   Compiling wasm-bindgen-macro-support v0.2.29
   Compiling wasm-bindgen-macro v0.2.29
   Compiling js-sys v0.3.6
   Compiling compiler-unhelpful v0.1.0 (…\compiler-unhelpful)
warning: unused variable: `operation`
  |
  = note: #[warn(unused_variables)] on by default

warning: unused variable: `index`

warning: unused variable: `array`

warning: struct is never constructed: `Renderer`
 --> src\lib.rs:4:1
  |
4 | struct Renderer;
  | ^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: method is never used: `push_operations`

    Finished dev [unoptimized + debuginfo] target(s) in 26.09s
