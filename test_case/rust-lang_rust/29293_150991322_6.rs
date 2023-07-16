 bash
$ RUST_BACKTRACE= gdb -- /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
GNU gdb (Gentoo 7.10 vanilla) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://bugs.gentoo.org/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
(gdb) run -VV
Starting program: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
warning: Cannot call inferior functions, Linux kernel PaX protection forbids return to non-executable pages!
[New LWP 31140]
error: Option 'version' given more than once.
^C
Program received signal SIGINT, Interrupt.
0x0000035c616709ad in ?? ()
(gdb) run -VVQuit
(gdb) thread apply all bt full

Thread 2 (LWP 31140):
#0  0x0000035c694cc986 in ?? ()
No symbol table info available.
#1  0x0000035c5fb20ca0 in ?? ()
No symbol table info available.
#2  0x0000035c694cc461 in ?? ()
No symbol table info available.
#3  0x0000035c694cc95e in ?? ()
No symbol table info available.
#4  0x0000000000000018 in ?? ()
No symbol table info available.
#5  0x0000000000000685 in ?? ()
No symbol table info available.
#6  0x0000035c5ce548e8 in ?? ()
No symbol table info available.
#7  0x0000035c5ce548e8 in ?? ()
No symbol table info available.
#8  0x000000000000011b in ?? ()
No symbol table info available.
#9  0x000000000000031f in ?? ()
No symbol table info available.
#10 0x0000035c5ce548e8 in ?? ()
---Type <return> to continue, or q <return> to quit---
No symbol table info available.
#11 0x0000035c5fb20cf0 in ?? ()
No symbol table info available.
#12 0x0000035c694cc56a in ?? ()
No symbol table info available.
#13 0x0000035c694cc95e in ?? ()
No symbol table info available.
#14 0x0000000000000018 in ?? ()
No symbol table info available.
#15 0x00000000000016ee in ?? ()
No symbol table info available.
#16 0x0000035c5ce3c708 in ?? ()
No symbol table info available.
#17 0x0000035c5ce64830 in ?? ()
No symbol table info available.
#18 0x0000000000001013 in ?? ()
No symbol table info available.
#19 0x00000000000016ee in ?? ()
No symbol table info available.
#20 0x0000035c5ce3c708 in ?? ()
No symbol table info available.
#21 0x0000035c5fb20d40 in ?? ()
No symbol table info available.
---Type <return> to continue, or q <return> to quit---
#22 0x0000035c694cc50d in ?? ()
No symbol table info available.
#23 0x0000035c694cc95e in ?? ()
No symbol table info available.
#24 0x0000000000000018 in ?? ()
No symbol table info available.
#25 0x0000000000004c32 in ?? ()
No symbol table info available.
#26 0x0000035c5cdb7e80 in ?? ()
No symbol table info available.
#27 0x0000035c5cdb7e80 in ?? ()
No symbol table info available.
#28 0x0000000000001ab6 in ?? ()
No symbol table info available.
#29 0x0000000000004c32 in ?? ()
No symbol table info available.
#30 0x0000035c5ce3c708 in ?? ()
No symbol table info available.
#31 0x0000035c5fb20d90 in ?? ()
No symbol table info available.
#32 0x0000035c694cc56a in ?? ()
No symbol table info available.
#33 0x0000035c694cc95e in ?? ()
---Type <return> to continue, or q <return> to quit---
No symbol table info available.
#34 0x0000000000000018 in ?? ()
No symbol table info available.
#35 0x000000000001d455 in ?? ()
No symbol table info available.
#36 0x0000035c5c96b000 in ?? ()
No symbol table info available.
#37 0x0000000000000000 in ?? ()
No symbol table info available.

Thread 1 (LWP 31020):
#0  0x0000035c616709ad in ?? ()
No symbol table info available.
#1  0x00000000697213a8 in ?? ()
No symbol table info available.
#2  0x0000000000000010 in ?? ()
No symbol table info available.
#3  0x0000035c616708d0 in ?? ()
No symbol table info available.
#4  0x0000035c5fb26d28 in ?? ()
No symbol table info available.
#5  0x0000000000000000 in ?? ()
No symbol table info available.
(gdb) quit
A debugging session is active.

    Inferior 1 [process 31020] will be killed.

Quit anyway? (y or n) y
