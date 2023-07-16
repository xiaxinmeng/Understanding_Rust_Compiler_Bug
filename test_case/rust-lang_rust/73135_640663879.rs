wasm
(func $__multi3 (type $t40) (param $p0 i32) (param $p1 i64) (param $p2 i64) (param $p3 i64) (param $p4 i64)
    (local $l5 i64) (local $l6 i64)
    local.get $p0
    local.get $p3
    i64.const 32
    i64.shr_u
    local.tee $l5
    local.get $p1
    i64.const 32
    i64.shr_u
    local.tee $l6
    i64.mul
    local.get $p3
    local.get $p2
    i64.mul
    i64.add
    local.get $p4
    local.get $p1
    i64.mul
    i64.add
    local.get $p3
    i64.const 4294967295
    i64.and
    local.tee $p3
    local.get $p1
    i64.const 4294967295
    i64.and
    local.tee $p1
    i64.mul
    local.tee $p4
    i64.const 32
    i64.shr_u
    local.get $p3
    local.get $l6
    i64.mul
    i64.add
    local.tee $p3
    i64.const 32
    i64.shr_u
    i64.add
    local.get $p3
    i64.const 4294967295
    i64.and
    local.get $l5
    local.get $p1
    i64.mul
    i64.add
    local.tee $p3
    i64.const 32
    i64.shr_u
    i64.add
    i64.store offset=8
    local.get $p0
    local.get $p3
    i64.const 32
    i64.shl
    local.get $p4
    i64.const 4294967295
    i64.and
    i64.or
    i64.store)
