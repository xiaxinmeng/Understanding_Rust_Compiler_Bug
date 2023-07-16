
  %14:gr64 = PHI %47:gr64, %bb.7, %56:gr64, %bb.8
  %64:gr64_nosp = SUB64ri8 %11:gr64_nosp(tied-def 0), 1, implicit-def $eflags
  %65:gr8 = SETCCr 2, implicit $eflags
  %60:gr32 = MOV32r0 implicit-def $eflags
  JCC_1 %bb.13, 2, implicit $eflags
