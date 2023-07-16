 terminal
$ rustc -g -opt0 blub.rs

$ gdb blub.exe
GNU gdb (GDB) 7.6.2
Copyright (C) 2013 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-w64-mingw32".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from U:\allgemein\rust_yaruml1\src\blub.exe...done.
(gdb) Traceback (most recent call last):
  File "<string>", line 3, in <module>
ImportError: No module named libstdcxx.v6.printers
c:\msys64\mingw64\bin\../etc/gdbinit:6: Error in sourced command file:
Error while executing Python code.
break rust_panic
Breakpoint 1 at 0x480c30
(gdb) run
Starting program: U:\allgemein\rust_yaruml1\src/blub.exe
[New Thread 3484.0xbe0]
task '<main>' panicked at 'called `Option::unwrap()` on a `None` value', D:\rust\src\libcore\option.rs:347

Breakpoint 1, 0x0000000000480c30 in rust_panic ()
(gdb) bt
#0  0x0000000000480c30 in rust_panic ()
#1  0x0000000000481561 in unwind::begin_unwind_inner::hd7e988ab0ca0dfc40Hc ()
#2  0x0000000000000008 in ?? ()
#3  0x0000000000000016 in ?? ()
#4  0x0000000000000000 in ?? ()
(gdb)
