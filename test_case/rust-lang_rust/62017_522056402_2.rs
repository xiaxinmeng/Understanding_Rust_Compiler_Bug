
  %0:gr64 = MOV64rm $noreg, 1, $noreg, target-flags(x86-gotpcrel) @_ZN1a6voidfn17h7d790e74ed3ff940E, $noreg
  ADJCALLSTACKDOWN64 0, 0, 0, implicit-def $rsp, implicit-def $eflags, implicit-def $ssp, implicit $rsp, implicit $ssp
  $rdi = COPY %0:gr64  
  CALL64pcrel32 @_ZN4main5takep17h851b97fbf6407c1bE, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b   $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit      $ssp, implicit $rdi
  ADJCALLSTACKUP64 0, 0, implicit-def $rsp, implicit-def $eflags, implicit-def $ssp, implicit $rsp, implicit $ssp
