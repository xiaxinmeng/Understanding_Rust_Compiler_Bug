asm
scratchpad::a: ; Doesn't allocate, calls write_str directly.
 lea     rdi, [rip, +, .L__unnamed_1]
 mov     esi, 11
 jmp     qword, ptr, [rip, +, write_str@GOTPCREL]
