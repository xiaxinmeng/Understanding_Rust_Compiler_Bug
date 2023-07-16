 shell
Breakpoint 1, test::b () at test.rs:2
2       { return 0; 
(gdb) n
3       } 
(gdb) n
test::a () at test.rs:6
6        } 
(gdb) n
test::main () at test.rs:9
9       }
(gdb) n
0x7f5590e0 in sys_common::unwind::try::try_fn::h16350519717893879347 ()
(gdb) n
Single stepping until exit from function _ZN10sys_common6unwind3try6try_fn21h16350519717893879347E,
which has no line number information.
0x7f559b30 in sys_common::unwind::inner_try::h68379cb749d9bf889Bt ()
(gdb) n
Single stepping until exit from function _ZN10sys_common6unwind9inner_try20h68379cb749d9bf889BtE,
which has no line number information.
0x7f558e48 in rt::lang_start::h17fda765239b3705kgA ()
(gdb) n
Single stepping until exit from function _ZN2rt10lang_start20h17fda765239b3705kgAE,
which has no line number information.
0x7f557c04 in main ()
(gdb) n
Single stepping until exit from function main,
which has no line number information.
__libc_start_main (main=0x7f557bc4 <main>, argc=1, argv=0xbeffef64, init=<optimized out>, 
    fini=0x7f56b999 <__libc_csu_fini>, rtld_fini=0xb6fea4c5 <_dl_fini>, stack_end=0xbeffef64)
    at libc-start.c:321
321     libc-start.c: No such file or directory.
[Inferior 1 (process 19920) exited normally]
