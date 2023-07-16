
bb.0 (%ir-block.0):
  successors: %bb.1(0x00000001), %bb.2(0x7fffffff); %bb.1(0.00%), %bb.2(100.00%)
  liveins: $rdi, $rsi
  %1:gr64 = COPY $rsi
  %0:gr64 = COPY $rdi
  TEST64rr %0:gr64, %0:gr64, implicit-def $eflags
  JCC_1 %bb.2, 5, implicit $eflags
  JMP_1 %bb.1


bb.1.bb2.i:
; predecessors: %bb.0
  ADJCALLSTACKDOWN64 0, 0, 0, implicit-def dead $rsp, implicit-def dead $eflags, implicit-def dead $ssp, implicit $rsp, implicit $ssp
  %2:gr64 = MOV32ri64 43
  %3:gr64 = IMPLICIT_DEF
  $rdi = COPY %3:gr64
  $rsi = COPY %2:gr64
  %4:gr64 = IMPLICIT_DEF
  $rdx = COPY %4:gr64
  CALL64m $rip, 1, $noreg, target-flags(x86-gotpcrel) @_ZN4core9panicking5panic17h68e56c2eeba99c8cE, $noreg, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit $rsi, implicit $rdx, implicit-def $rsp, implicit-def $ssp :: (load 8 from got)
  ADJCALLSTACKUP64 0, 0, implicit-def dead $rsp, implicit-def dead $eflags, implicit-def dead $ssp, implicit $rsp, implicit $ssp


bb.2._ZN4core6option15Option$LT$T$GT$6unwrap17h2ccd910ff0fb2487E.exit:
; predecessors: %bb.0
  $rax = COPY %1:gr64
  RET 0, $rax
