wasm
(func (;0;) (type 0) (param i32 i32) (result i32)
    local.get 1
    i32.load8_u
    local.get 0
    i32.load8_u
    i32.add
    i32.extend8_s)
