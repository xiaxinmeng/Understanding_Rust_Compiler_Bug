
$ cat /etc/centos-release
CentOS release 6.10 (Final)
$ ./memcpy_el6.run
person_copy: Pierre de Fermat, 46 
$ cat /etc/centos-release
CentOS Linux release 7.7.1908 (Core)
$ ./memcpy_el6.run
person_copy: Pierre de Fermat, 46 
$ objdump -T memcpy_el6.run

memcpy_el6.run:     file format elf64-x86-64

DYNAMIC SYMBOL TABLE:
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 printf
0000000000000000  w   D  *UND*	0000000000000000              __gmon_start__
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 __libc_start_main
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 strlen
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 memcpy
$ objdump -T memcpy_el7.0.run

memcpy_el7.0.run:     file format elf64-x86-64

DYNAMIC SYMBOL TABLE:
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 strlen
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 printf
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 __libc_start_main
0000000000000000  w   D  *UND*	0000000000000000              __gmon_start__
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.14  memcpy
$ cat /etc/centos-release
CentOS release 6.10 (Final)
$ ./memcpy_el7.0.run
./memcpy_el7.0.run: /lib64/libc.so.6: version `GLIBC_2.14' not found (required by ./memcpy_el7.0.run)
