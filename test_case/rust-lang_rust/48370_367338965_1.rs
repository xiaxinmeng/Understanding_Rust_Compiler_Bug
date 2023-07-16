
[santiago@archlinux tmp]$ valgrind --tool=massif ~/src/oss/rust1/build/x86_64-unknown-linux-gnu/stage1/bin/rustc +stage1 test.rs 
==11363== Massif, a heap profiler
==11363== Copyright (C) 2003-2017, and GNU GPL'd, by Nicholas Nethercote
==11363== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==11363== Command: /home/santiago/src/oss/rust1/build/x86_64-unknown-linux-gnu/stage1/bin/rustc +stage1 -g test.rs
==11363== 
error: multiple input filenames provided

==11363== 
