
Process 14641 launched: './segv' (x86_64)
Process 14641 stopped
* thread #1: tid = 0x1c03, 0x0000000100002e98 segv`Box$LT$BoxedFoo$u{20}$u{2b}$u{20}$u{27}static$GT$::glue_drop.1583::h37e40b03ca5bab2f + 72, stop reason = EXC_BAD_ACCESS (code=1, address=0x10)
    frame #0: 0x0000000100002e98 segv`Box$LT$BoxedFoo$u{20}$u{2b}$u{20}$u{27}static$GT$::glue_drop.1583::h37e40b03ca5bab2f + 72
segv`Box$LT$BoxedFoo$u{20}$u{2b}$u{20}$u{27}static$GT$::glue_drop.1583::h37e40b03ca5bab2f + 72:
-> 0x100002e98:  movq   (%rcx), %rcx
   0x100002e9b:  movq   %rax, %rdi
   0x100002e9e:  callq  *%rcx
   0x100002ea0:  jmp    0x100002e8a               ; Box$LT$BoxedFoo$u{20}$u{2b}$u{20}$u{27}static$GT$::glue_drop.1583::h37e40b03ca5bab2f + 58
