sh
$ rustc +stable foo.rs --crate-type cdylib --target wasm32-unknown-unknown -O
$ wasm2wat foo.wasm | rg import
$ rustc +stable foo.rs --crate-type cdylib --target wasm32-unknown-unknown -O -C lto
$ wasm2wat foo.wasm | rg import
$ rustc +beta foo.rs --crate-type cdylib --target wasm32-unknown-unknown -O
$ wasm2wat foo.wasm | rg import
$ rustc +beta foo.rs --crate-type cdylib --target wasm32-unknown-unknown -O -C lto
$ wasm2wat foo.wasm | rg import
  (import "env" "_ZN4core9panicking18panic_bounds_check17hadfb911a22557eccE" (func $_ZN4core9panicking18panic_bounds_check17hadfb911a22557eccE (type 0)))
