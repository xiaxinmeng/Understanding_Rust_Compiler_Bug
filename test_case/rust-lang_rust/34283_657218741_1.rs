asm
testcase:                               # @testcase
  .cfi_startproc
# %bb.0:
                                          # kill: def $dil killed $dil killed $edi
  movb>-%dil, -1(%rsp)          # 1-byte Spill
  jmp>.LBB15_1
.LBB15_1:                               # %bb2
  movb>--1(%rsp), %al           # 1-byte Reload
  retq
