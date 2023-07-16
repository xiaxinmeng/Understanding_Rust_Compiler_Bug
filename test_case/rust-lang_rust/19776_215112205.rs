 bash
$ ./configure --disable-elf-tls
$ make -j6
$ ./x86_64-unknown-linux-gnu/stage2/bin/rustc learn.rs 
$  valgrind --leak-check=full ./learn
==14462== Memcheck, a memory error detector
==14462== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==14462== Using Valgrind-3.10.0 and LibVEX; rerun with -h for copyright info
==14462== Command: ./learn
==14462== 
Hello World!
How are you today
==14462== 
==14462== HEAP SUMMARY:
==14462==     in use at exit: 0 bytes in 0 blocks
==14462==   total heap usage: 22 allocs, 22 frees, 2,680 bytes allocated
==14462== 
==14462== All heap blocks were freed -- no leaks are possible
==14462== 
==14462== For counts of detected and suppressed errors, rerun with: -v
==14462== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
$ ./x86_64-unknown-linux-gnu/stage2/bin/rustc --version
rustc 1.10.0-dev (80bff1eea 2016-04-26)
