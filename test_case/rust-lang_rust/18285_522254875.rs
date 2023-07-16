
GNU gdb (GDB) 8.3
Copyright (C) 2019 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

    For help, type "help".
    Type "apropos word" to search for commands related to "word"...
    Reading symbols from ./test_macro...
    (gdb) run
    Starting program: /home/san/projects/rust/playground/test_macro
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/usr/lib/libthread_db.so.1".

    Program received signal SIGSEGV, Segmentation fault.
    main::bar () at src/main.rs:6
    6               *(0 as *mut u32) = 0;
    (gdb) bt
    #0  main::bar () at src/main.rs:6
    #1  0x0000555555558256 in main::foo () at src/main.rs:11
    #2  0x0000555555558276 in main::main () at src/main.rs:17
    (gdb)
