
(gdb) bt
#0  0x54b28d34 in main ()
(gdb) next
Single stepping until exit from function main,
which has no line number information.
0x54dce7cc in std::rt::lang_start::hb072ea7464d45b94 ()
(gdb) bt
#0  0x54dce7cc in std::rt::lang_start::hb072ea7464d45b94 ()
#1  0x76e12294 in __libc_start_main (main=0x7efff5e4, argc=1995665408, argv=0x76e12250 <__libc_start_main+208>, init=<optimized out>, 
    fini=0x54df9615 <__libc_csu_fini>, rtld_fini=0x76fdf408 <_dl_fini>, stack_end=0x7efff5e4) at libc-start.c:287
#2  0x54aed04e in _start ()
(gdb) next
Single stepping until exit from function _ZN3std2rt10lang_start17hb072ea7464d45b94E,
which has no line number information.
[Inferior 1 (process 31582) exited with code 01]
(gdb) bt
No stack.
(gdb) 
