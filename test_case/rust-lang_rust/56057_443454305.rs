
      t2: i64,ch = CopyFromReg t0, Register:i64 %0
            t4: i64,ch = CopyFromReg t0, Register:i64 %1
          t7: i64 = srl t4, Constant:i8<4>
        t9: i64 = and t7, Constant:i64<3>
      t10: i64 = shl t9, Constant:i64<4>
    t11: i64 = add t2, t10
