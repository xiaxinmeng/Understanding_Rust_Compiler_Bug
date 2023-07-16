
^C[Thread 0xffffdabd4e90 (LWP 24577) exited]
[Thread 0xffffdb1d7e90 (LWP 24535) exited]
[Thread 0xffffdb3d8e90 (LWP 24527) exited]
[Thread 0xffffdadd5e90 (LWP 24571) exited]

Thread 1 "cargo" received signal SIGINT, Interrupt.
0x0000aaaaab161f44 in ?? ()
(gdb) bt
#0  0x0000aaaaab161f44 in ?? ()
#1  0x0000aaaaaab888dc in ?? ()
#2  0x0000aaaaaae145dc in ?? ()
#3  0x0000aaaaaaeaeda0 in ?? ()
#4  0x0000aaaaaae3b05c in ?? ()
#5  0x0000aaaaaaeafd98 in ?? ()
#6  0x0000aaaaab163bdc in ?? ()
#7  0x0000aaaaaab9d1ec in ?? ()
#8  0x0000aaaaaae395c0 in ?? ()
#9  0x0000aaaaaabd7b54 in ?? ()
#10 0x0000aaaaaac361e8 in ?? ()
#11 0x0000aaaaaac31ed8 in ?? ()
#12 0x0000aaaaaab30650 in ?? ()
#13 0x0000aaaaaab44910 in ?? ()
#14 0x0000aaaaaab869b4 in ?? ()
#15 0x0000aaaaaab6b83c in ?? ()
#16 0x0000aaaaab14942c in ?? ()
#17 0x0000aaaaab163bdc in ?? ()
#18 0x0000aaaaab14ee1c in ?? ()
#19 0x0000aaaaaab888bc in ?? ()
#20 0x0000fffff7a6e6e0 in __libc_start_main () from /lib/aarch64-linux-gnu/libc.so.6
#21 0x0000aaaaaab281dc in ?? ()
Backtrace stopped: not enough registers or memory available to unwind further
(gdb) 

