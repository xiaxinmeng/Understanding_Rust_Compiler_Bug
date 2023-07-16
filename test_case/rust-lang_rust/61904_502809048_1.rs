bash
% gdb hello_gcc
Reading symbols from a.out...(no debugging symbols found)...done.
(gdb) disas main
Dump of assembler code for function main:
   0x0000000000001130 <+0>:     push   rbp
   0x0000000000001131 <+1>:     mov    rbp,rsp
   0x0000000000001134 <+4>:     lea    rdi,[rip+0x2ec6]        # 0x4001 <s>
   0x000000000000113b <+11>:    call   0x1210 <puts@plt>
   0x0000000000001140 <+16>:    mov    eax,0x0
   0x0000000000001145 <+21>:    pop    rbp
   0x0000000000001146 <+22>:    ret
End of assembler dump.
(gdb) x/s 0x4001
0x4001 <s>:     ""
(gdb) quit
