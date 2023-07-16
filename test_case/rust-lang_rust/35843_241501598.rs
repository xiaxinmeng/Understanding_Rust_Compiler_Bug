
(gdb) run
Starting program: /usr/local/bin/cargo 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x00000000 in ?? ()
(gdb) bt full
#0  0x00000000 in ?? ()
No symbol table info available.
#1  0x76f7e7b4 in __pthread_initialize_minimal_internal () from /usr/lib/libpthread.so.0
No symbol table info available.
#2  0x76f7dd54 in _init () from /usr/lib/libpthread.so.0
No symbol table info available.
#3  0x76fdf2e8 in call_init.part () from /lib/ld-linux-armhf.so.3
No symbol table info available.
#4  0x76fdf4b8 in _dl_init () from /lib/ld-linux-armhf.so.3
No symbol table info available.
#5  0x76fcfbc4 in _dl_start_user () from /lib/ld-linux-armhf.so.3
No symbol table info available.
Backtrace stopped: previous frame identical to this frame (corrupt stack?)
(gdb)
