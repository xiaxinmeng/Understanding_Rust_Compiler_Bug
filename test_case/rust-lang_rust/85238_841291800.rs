
Do you need "set solib-search-path" or "set sysroot"?

Program received signal SIGILL, Illegal instruction.
0x00a87060 in OPENSSL_crypto207_probe ()
(gdb) continue
Continuing.

Program received signal SIGILL, Illegal instruction.
0x00a87084 in OPENSSL_madd300_probe ()
(gdb) continue
Continuing.
process 697 is executing new program: /root/.rustup/toolchains/stable-powerpc-unknown-linux-gnu/bin/rustc
warning: Could not load symbols for executable /root/.rustup/toolchains/stable-powerpc-unknown-linux-gnu/bin/rustc.
Do you need "set sysroot"?
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
Warning:
Cannot insert breakpoint -2.
Cannot access memory at address 0x4a3790
