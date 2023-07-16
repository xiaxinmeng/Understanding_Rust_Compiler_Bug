
D:\>D:\program\TDM-GCC-32\bin\gdb.exe test.exe
GNU gdb (GDB) 7.6.1
Copyright (C) 2013 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "mingw32".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from D:\test.exe...done.
(gdb) break 1
Breakpoint 1 at 0x401613: file test.rs, line 1.
(gdb) r
Starting program: D:\test.exe
[New Thread 3744.0x19d0]
[New Thread 3744.0x19e4]

Program received signal SIGSEGV, Segmentation fault.
0x7795e12c in ?? ()
