
(gdb) disassemble
[...]
   0x00007ffff7f5582f <+3759>:	lea    0x6a8da(%rip),%rcx        # 0x7ffff7fc0110 <__PRETTY_FUNCTION__.1>
   0x00007ffff7f55836 <+3766>:	mov    $0x76,%edx
   0x00007ffff7f5583b <+3771>:	lea    0x6a738(%rip),%rsi        # 0x7ffff7fbff7a
   0x00007ffff7f55842 <+3778>:	lea    0x6a744(%rip),%rdi        # 0x7ffff7fbff8d
   0x00007ffff7f55849 <+3785>:	call   0x7ffff7f0bde0 <__assert_fail>
(gdb) x/s 0x7ffff7fbff8d
0x7ffff7fbff8d:	"info[DT_RUNPATH] == NULL"
