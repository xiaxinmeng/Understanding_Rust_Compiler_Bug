rust
% "/usr/bin/gdb" "-quiet" "-nx" "-command=/home/lzutao/forked/rust/compiler-rust/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string/empty-string.debugger.script"       
GNU gdb (Debian 7.12-6) 7.12.0.20161007-git
Copyright (C) 2016 Free Software Foundation, Inc.
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
Type "apropos word" to search for commands related to "word".
Traceback (most recent call last):
  File "/home/lzutao/forked/rust/compiler-rust/src/etc/gdb_load_rust_pretty_printers.py", line 2, in <module>
    import gdb_rust_pretty_printing
ImportError: No module named 'gdb_rust_pretty_printing'
Breakpoint 1 at 0x263f: file /home/lzutao/forked/rust/compiler-rust/src/test/debuginfo/empty-string.rs, line 32.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, empty_string::main::h4739d54fd70936a5 () at /home/lzutao/forked/rust/compiler-rust/src/test/debuginfo/empty-string.rs:32
32          zzz(); // #break
$1 = alloc::string::String {vec: alloc::vec::Vec<u8> {buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {ptr: core::ptr::unique::Unique<u8> {pointer: 0x1 <error: Cannot access memory at address 0x1>, _marker: core::marker::PhantomData<u8>}, cap: 0, a: alloc::alloc::Gl
obal}, len: 0}}
$2 = &str {data_ptr: 0x5555555549fe "attempt to create unaligned or null slicesrc/libcore/slice/mod.rsattempt to create slice covering half the address space\000", length: 0}
A debugging session is active.

        Inferior 1 [process 14010] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
