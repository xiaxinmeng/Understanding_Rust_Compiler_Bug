
LLVM ERROR: Cannot select: t9: i16 = addrspacecast[1 -> 0] t8
  t8: i16,ch = load<(load 2 from %ir.5, align 1, !alias.scope !265)> t0, t7, undef:i16
    t7: i16,ch = load<(dereferenceable load 2 from %ir.4, align 1)> t0, t2, undef:i16
      t2: i16,ch = CopyFromReg t0, Register:i16 %19
        t1: i16 = Register %19
      t6: i16 = undef
    t6: i16 = undef
In function: _ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h2dc31f322889dd28E
