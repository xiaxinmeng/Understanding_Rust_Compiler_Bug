
test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ wat2wasm test.wat

test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ wasm2wat test.wasm
(module
  (type (;0;) (func (param i32 i32) (result i32 i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32 i32)
    local.get 0
    local.get 1
    i32.mul
    local.get 0
    local.get 1
    i32.sub)
  (export "magic" (func 0)))
