
Program received signal SIGSEGV, Segmentation fault.
0x800167ee in je_malloc_tsd_boot0.part ()
(gdb) bt
#0  0x800167ee in je_malloc_tsd_boot0.part ()
#1  0x80016657 in je_malloc_tsd_boot0 ()
#2  0x80039975 in malloc_init_hard.lto_priv ()
#3  0x800383fe in jemalloc_constructor ()
#4  0x8004f4f6 in __libc_csu_init ()
#5  0xb7dec237 in __libc_start_main () from /lib/libc.so.6
#6  0x8000aa81 in _start () at ../sysdeps/i386/start.S:102
