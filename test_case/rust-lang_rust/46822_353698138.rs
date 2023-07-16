
(gdb) run
Starting program: /home/sam/hello-world
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabi/libthread_db.so.1".
Hello, world!

Program received signal SIGSEGV, Segmentation fault.
0x00410644 in __sync_val_compare_and_swap_4 ()
(gdb) backtrace
#0  0x00410644 in __sync_val_compare_and_swap_4 ()
#1  0x00407ec4 in std::rt::lang_start::hc79ba98377dc1008 ()
#2  0xb6e1c2cc in __libc_start_main (main=0xbefffa74, argc=-1225486336, argv=0xb6e1c2cc <__libc_start_main+280>, init=<optimized out>,
    fini=0x410b48 <__libc_csu_fini>, rtld_fini=0xb6fdfc60 <_dl_fini>, stack_end=0xbefffa74) at libc-start.c:287
#3  0x00401d1c in _start ()
(gdb) disas
Dump of assembler code for function __sync_val_compare_and_swap_4:
   0x00410634 <+0>:     push    {r4, r5, r6, lr}
   0x00410638 <+4>:     mov     r4, r2
   0x0041063c <+8>:     mov     r6, r1
   0x00410640 <+12>:    mov     r5, r0
=> 0x00410644 <+16>:    ldr     r0, [r4]
   0x00410648 <+20>:    cmp     r0, r5
   0x0041064c <+24>:    popne   {r4, r5, r6, pc}
   0x00410650 <+28>:    ldr     r3, [pc, #28]   ; 0x410674 <__sync_val_compare_and_swap_4+64>
   0x00410654 <+32>:    mov     r0, r5
   0x00410658 <+36>:    mov     r1, r6
   0x0041065c <+40>:    mov     r2, r4
   0x00410660 <+44>:    blx     r3
   0x00410664 <+48>:    cmp     r0, #0
   0x00410668 <+52>:    bne     0x410644 <__sync_val_compare_and_swap_4+16>
   0x0041066c <+56>:    mov     r0, r5
   0x00410670 <+60>:    pop     {r4, r5, r6, pc}
   0x00410674 <+64>:                    ; <UNDEFINED> instruction: 0xffff0fc0
End of assembler dump.
