
# After Instruction Selection
# Machine code for function test: IsSSA, TracksLiveness
Function Live Ins: $rdi in %0

bb.0 (%ir-block.0):
  liveins: $rdi
  %0:gr64 = COPY $rdi
  %1:gr64 = COPY killed %0:gr64
  ADJCALLSTACKDOWN64 0, 0, 0, implicit-def $rsp, implicit-def $eflags, implicit-def $ssp, implicit $rsp, implicit $ssp
  CALL64pcrel32 target-flags(x86-plt) @get, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit-def $rax, implicit-def $rdx
  ADJCALLSTACKUP64 0, 0, implicit-def $rsp, implicit-def $eflags, implicit-def $ssp, implicit $rsp, implicit $ssp
  %7:gr64 = COPY $rax
  %8:gr64 = COPY $rdx
  MOV64mr %1:gr64, 1, $noreg, 0, $noreg, killed %8:gr64 :: (store 8 into %ir.p)
  %2:gr64 = IMPLICIT_DEF
  ADJCALLSTACKDOWN64 0, 0, 0, implicit-def $rsp, implicit-def $eflags, implicit-def $ssp, implicit $rsp, implicit $ssp
  $rdi = COPY %2:gr64
  $rsi = COPY %8:gr64
  CALL64pcrel32 target-flags(x86-plt) @use, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit $rsi
  ADJCALLSTACKUP64 0, 0, implicit-def $rsp, implicit-def $eflags, implicit-def $ssp, implicit $rsp, implicit $ssp
  RETQ

# End machine code for function test.

*** Bad machine code: Using a killed virtual register ***
- function:    test
- basic block: %bb.0  (0x557dd5832118)
- instruction: $rsi = COPY %8:gr64
- operand 1:   %8:gr64
