
root@localhost:~/rust# gdb /root/.rustup/toolchains/stable-powerpc-unknown-linux-gnu/bin/rustc
GNU gdb (GDB) 10.1
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "powerpc64-t2-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /root/.rustup/toolchains/stable-powerpc-unknown-linux-gnu/bin/rustc...
(gdb) run ./hello.rs
Starting program: /root/.rustup/toolchains/stable-powerpc-unknown-linux-gnu/bin/rustc ./hello.rs
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
warning: Could not load shared library symbols for 9 libraries, e.g. /root/.rustup/toolchains/stable-powerpc-unknown-linux-gnu/bin/../lib/librustc_driver-f97d9e14fc618140.so.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
[New LWP 977]

Thread 2 "rustc" received signal SIGILL, Illegal instruction.
[Switching to LWP 977]
0x460db18c in ?? ()
(gdb) backtrace
#0  0x460db18c in ?? ()
#1  0x40b45704 in ?? ()
#2  0x40abfc98 in ?? ()
#3  0x40ab3734 in ?? ()
#4  0x40ac0e48 in ?? ()
#5  0x40aeb104 in ?? ()
#6  0x40aecac0 in ?? ()
#7  0x40af9348 in ?? ()
#8  0x00302da0 in ?? ()
#9  0x001e0418 in ?? ()
#10 0x48872d0c in ?? ()
(gdb) disassemble $pc-16,+32
Dump of assembler code from 0x460db17c to 0x460db19c:
   0x460db17c:  mtlr    r0
   0x460db180:  mtctr   r12
   0x460db184:  bctr
   0x460db188:  bl      0x460db190
=> 0x460db18c:  .long 0x2671c64
   0x460db190:  mflr    r3
   0x460db194:  lwz     r4,0(r3)
   0x460db198:  add     r3,r4,r3
End of assembler dump.
(gdb)
