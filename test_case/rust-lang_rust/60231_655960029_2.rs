wat
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func))
  (import "env" "memory" (memory (;0;) 0))
  (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
  (import "env" "__stack_pointer" (global (;0;) (mut i32)))
  (import "env" "__memory_base" (global (;1;) i32))
  (import "env" "__table_base" (global (;2;) i32))
  (import "import" "foo" (func $_ZN3lib3foo17hb22c31d0714767f1E (type 0)))
  (func $__wasm_call_ctors (type 1)
    call $__wasm_apply_relocs)
  (func $__wasm_apply_relocs (type 1))
  (func $main (type 1)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 5
    i32.store8 offset=13
    local.get 0
    i32.const 13
    i32.add
    call $_ZN3lib3foo17hb22c31d0714767f1E
    local.get 0
    i32.const 26984
    i32.store16 offset=14 align=1
    local.get 0
    i32.const 14
    i32.add
    call $_ZN3lib3foo17hb22c31d0714767f1E
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (export "main" (func $main)))
