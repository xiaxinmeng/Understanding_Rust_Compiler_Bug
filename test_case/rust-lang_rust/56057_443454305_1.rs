
      t2: i64,ch = CopyFromReg t0, Register:i64 %0
        t4: i64,ch = CopyFromReg t0, Register:i64 %1
      t26: i64 = and t4, Constant:i64<48>
    t11: i64 = add t2, t26
