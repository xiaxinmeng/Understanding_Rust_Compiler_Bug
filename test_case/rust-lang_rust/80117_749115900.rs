
mk7i-suse-13-2-rajput_mk7_~/development/Rajat/misc$ gdb ./simple_print_i585_musl
GNU gdb (GDB; openSUSE 13.2) 7.8
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "i586-suse-linux".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://bugs.opensuse.org/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...

warning: /etc/gdbinit.d/gdb-heap.py: No such file or directory
Reading symbols from ./simple_print_i585_musl...done.
warning: Missing auto-load scripts referenced in section .debug_gdb_scripts
of file /home/mk7/development/Rajat/misc/simple_print_i585_musl
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) run
Starting program: /home/mk7/development/Rajat/misc/simple_print_i585_musl
guess the number!

Program terminated with signal SIGSEGV, Segmentation fault.
The program no longer exists.
(gdb) bt
No stack.
(gdb) q
