
  EH_LABEL <mcsymbol .Ltmp0>
  ADJCALLSTACKDOWN64 0, 0, 0, implicit-def dead $rsp, implicit-def dead $eflags, implicit-def dead $ssp, implicit $rsp, implicit $ssp
  %0:gr64 = MOV64rm $rip, 1, $noreg, target-flags(x86-gotpcrel) @_ZN1a6voidfn17h7d790e74ed3ff940E, $noreg :: (load 8 from got)
  $rdi = COPY %0:gr64
  CALL64pcrel32 @_ZN4main5takep17h851b97fbf6407c1bE, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b   $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit      $ssp, implicit $rdi, implicit-def $rsp, implicit-def $ssp
  ADJCALLSTACKUP64 0, 0, implicit-def dead $rsp, implicit-def dead $eflags, implicit-def dead $ssp, implicit $rsp, implicit $ssp
  EH_LABEL <mcsymbol .Ltmp1>
  JMP_1 %bb.2
