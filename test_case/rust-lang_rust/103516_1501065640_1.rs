wat
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (result i32)))
  (func $_ZN3foo3add17hc001cc2609bca236E (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add)
  (func $add_ptr (type 1) (result i32)
    i32.const 1)
  (table (;0;) 2 2 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "add_ptr" (func $add_ptr))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN3foo3add17hc001cc2609bca236E))
