wast
  (func (;1;) (type 0) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    i32.const 0
    call 0
    local.get 0
    local.get 1
    i64.load offset=8
    i64.store
    local.get 1
    i32.const 32
    i32.add
    global.set 0)
