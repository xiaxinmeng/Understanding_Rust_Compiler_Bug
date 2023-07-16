
   (block $switch-case0
     (block $switch-case
      (br_table $switch-case $switch-case0 $switch-default
       (i32.wrap/i64
        (i64.sub
         (get_local $$0)
         (i64.const 9218868437227405312)
        )
       )
      )
     )
     (block
      (call $__ZN3std9panicking15begin_panic_new17hd7ebc2b159ba3f26E
       (i32.const 5257)
       (i32.const 1)
       (i32.const 3232)
      )
      (br $switch)
     )
    )
