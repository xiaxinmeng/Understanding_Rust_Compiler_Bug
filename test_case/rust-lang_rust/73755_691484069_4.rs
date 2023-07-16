wat
(module
  (func $magic
    (export "magic")
    (param i32 i32)
    (result i32 i32)

    local.get 0
    local.get 1
    i32.mul

    local.get 0
    local.get 1
    i32.sub
  )
)
