
root@deb4g:~# gdb /usr/local/bin/rustc
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
Reading symbols from /usr/local/bin/rustc...(no debugging symbols found)...done.
(gdb) r /root/hello.rs                                                                                                                                                      
Starting program: /usr/local/bin/rustc /root/hello.rs                                                                                                                       
[Thread debugging using libthread_db enabled]                                                                                                                               
Using host libthread_db library "/lib/sparc64-linux-gnu/libthread_db.so.1".
[New Thread 0xfff80001097bd910 (LWP 115392)]

Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0xfff80001097bd910 (LWP 115392)]
0xfff80001076ae2b8 in __ctzdi2 () from /usr/local/bin/../lib/../lib/../lib/librustc_llvm-2cab755ba8bcc047.so
(gdb) bt
#0  0xfff80001076ae2b8 in __ctzdi2 () from /usr/local/bin/../lib/../lib/../lib/librustc_llvm-2cab755ba8bcc047.so
#1  0xfff80001076ae2e8 in __ctzdi2 () from /usr/local/bin/../lib/../lib/../lib/librustc_llvm-2cab755ba8bcc047.so
(gdb) quit
A debugging session is active.

        Inferior 1 [process 115389] will be killed.

Quit anyway? (y or n) y
root@deb4g:~#
