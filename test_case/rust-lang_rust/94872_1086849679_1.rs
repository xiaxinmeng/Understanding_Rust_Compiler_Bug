
$ PATH=/d/tmp/generic-rust-windows/bin:$PATH

$ rustc ../Projekty/rust/src/test/ui/abi/abi-sysv64-register-usage.rs

$ ./abi-sysv64-register-usage.exe
Segmentation fault

$ echo $?
139

$ gdb abi-sysv64-register-usage.exe
GNU gdb (GDB) 11.2
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-w64-mingw32".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from abi-sysv64-register-usage.exe...
(gdb) r
Starting program: D:\tmp\abi-sysv64-register-usage.exe
[New Thread 1760.0x1460]
[New Thread 1760.0x802c]
[New Thread 1760.0x5c64]

Thread 1 received signal SIGSEGV, Segmentation fault.
0x00007ff65d0ec548 in std::sys_common::thread_info::THREAD_INFO::__getit::h685daeac7503b1c0 ()
(gdb) bt
#0  0x00007ff65d0ec548 in std::sys_common::thread_info::THREAD_INFO::__getit::h685daeac7503b1c0 ()
#1  0x00007ff65d1114b8 in std::thread::local::LocalKey<T>::with ()
#2  0x00007ff65d0f9ce8 in std::rt::lang_start_internal ()
#3  0x00007ff65d0f9c90 in std::rt::lang_start ()
#4  0x00007ff65d189ebc in main ()
