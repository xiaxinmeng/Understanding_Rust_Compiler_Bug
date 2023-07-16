
Program received signal SIGSEGV, Segmentation fault.
0x00007ffff7edd328 in __memmove_avx_unaligned_erms () from /usr/lib/libc.so.6
(gdb) bt
#0  0x00007ffff7edd328 in __memmove_avx_unaligned_erms () from /usr/lib/libc.so.6
#1  0x000055555558856b in write_bytes (s=0x5555555b7296 "_ZN5alloc11collections5btree4node25Handle$LT$Node$C$Type$GT$9into_node17hf4012ff21ae14a7aE", len=90) at /rustc/f509b26a7730d721ef87423a72b3fdf8724b4afa/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:178
#2  write_string (s=0x5555555b7296 "_ZN5alloc11collections5btree4node25Handle$LT$Node$C$Type$GT$9into_node17hf4012ff21ae14a7aE") at /rustc/f509b26a7730d721ef87423a72b3fdf8724b4afa/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:202
#3  llvm_gcda_emit_function (ident=<optimized out>, function_name=0x5555555b7296 "_ZN5alloc11collections5btree4node25Handle$LT$Node$C$Type$GT$9into_node17hf4012ff21ae14a7aE", func_checksum=<optimized out>, use_extra_checksum=0 '\000', cfg_checksum=<optimized out>)
    at /rustc/f509b26a7730d721ef87423a72b3fdf8724b4afa/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:455
#4  0x0000555555568670 in __llvm_gcov_writeout ()
#5  0x00005555555894b3 in llvm_writeout_files () at /rustc/f509b26a7730d721ef87423a72b3fdf8724b4afa/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:606
#6  0x00007ffff7db7537 in __run_exit_handlers () from /usr/lib/libc.so.6
#7  0x00007ffff7db76ee in exit () from /usr/lib/libc.so.6
#8  0x00007ffff7da002a in __libc_start_main () from /usr/lib/libc.so.6
#9  0x000055555555f24e in _start ()
