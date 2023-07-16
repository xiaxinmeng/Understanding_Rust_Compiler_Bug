
    t2: i64,ch = CopyFromReg t0, Register:i64 %0
            t4: i32,ch = CopyFromReg t0, Register:i32 %1
          t7: i32 = srl t4, Constant:i8<4>
        t9: i32 = and t7, Constant:i32<3>
      t10: i64 = zero_extend t9
    t12: i64 = shl t10, Constant:i64<4>
  t13: i64 = add t2, t12
