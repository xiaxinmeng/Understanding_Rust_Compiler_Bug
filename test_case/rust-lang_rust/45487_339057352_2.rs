
root@deb4g:~# gdb ./bugtest 
GNU gdb (Debian 7.12-6) 7.12.0.20161007-git
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "sparc64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./bugtest...done.
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /root/bugtest.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) r
Starting program: /root/bugtest 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/sparc64-linux-gnu/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x000001000002a4fc in __ctzdi2 ()
(gdb) bt
#0  0x000001000002a4fc in __ctzdi2 ()
#1  0x000001000002a52c in __ctzdi2 ()                                                                                                                                       
Backtrace stopped: previous frame inner to this frame (corrupt stack?)                                                                                                      
(gdb)
