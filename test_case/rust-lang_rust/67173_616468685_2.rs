
$ rpm -qi glibc |grep -iE "(name|version|release)"
Name        : glibc
Version     : 2.17
Release     : 292.el7
$ objdump -T /lib64/libc.so.6|grep -i memcpy
00000000000a8b10  w   DF .text	0000000000000009  GLIBC_2.2.5 wmemcpy
0000000000116f10 g    DF .text	0000000000000014  GLIBC_2.4   __wmemcpy_chk
0000000000094a80 g   iD  .text	0000000000000055  GLIBC_2.14  memcpy
000000000008f870 g   iD  .text	000000000000004b (GLIBC_2.2.5) memcpy
00000000001151a0 g   iD  .text	0000000000000055  GLIBC_2.3.4 __memcpy_chk
$ objdump -T memcpy_el7.run

memcpy_el7.run:     file format elf64-x86-64

DYNAMIC SYMBOL TABLE:
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 memcpy
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 strlen
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 prin
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 __libc_start_main
0000000000000000  w   D  *UND*	0000000000000000              __gmon_start__
