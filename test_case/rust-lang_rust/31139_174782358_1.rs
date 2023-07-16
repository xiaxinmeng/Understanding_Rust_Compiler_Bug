
$ gdb ./foo
GNU gdb (Ubuntu 7.7.1-0ubuntu5~14.04.2) 7.7.1
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./foo...done.
(gdb) r
Starting program: /home/alex/code/wut/wut/rustc-nightly-i686-unknown-linux-gnu/rustc/foo 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x000055555555b324 in core..option..Option$LT$core..result..Result$LT$$LP$$RP$$C$$u20$Box$LT$core..any..Any$u20$$u2b$$u20$Send$u20$$u2b$$u20$$u27$static$GT$$GT$$GT$::drop.3726::hc2f93767862c1e08 ()
(gdb) disas
Dump of assembler code for function _ZN142core..option..Option$LT$core..result..Result$LT$$LP$$RP$$C$$u20$Box$LT$core..any..Any$u20$$u2b$$u20$Send$u20$$u2b$$u20$$u27$static$GT$$GT$$GT$9drop.372617hc2f93767862c1e08E:
   0x000055555555b320 <+0>:     sub    $0x18,%rsp
=> 0x000055555555b324 <+4>:     mov    (%rdi),%rax
   0x000055555555b327 <+7>:     test   %rax,%rax
   0x000055555555b32a <+10>:    mov    %rdi,0x10(%rsp)
   0x000055555555b32f <+15>:    mov    %rax,0x8(%rsp)
(gdb) print/x $rdi
$1 = 0x1d1d1d1d1d1d1d2d
