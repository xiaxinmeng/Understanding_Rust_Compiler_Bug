plain
test [debuginfo-gdb] src/test/debuginfo/cross-crate-type-uniquing.rs ... ok

failures:

---- [debuginfo-gdb] src/test/debuginfo/basic-types-globals.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = false
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/i686-unknown-linux-gnu/test/debuginfo/basic-types-globals.gdb/basic-types-globals.debugger.script"
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x737: file /checkout/src/test/debuginfo/basic-types-globals.rs, line 74.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_globals::main::hbc8d68058c8fd353 () at /checkout/src/test/debuginfo/basic-types-globals.rs:74
74     _zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/i686-unknown-linux-gnu/test/debuginfo/basic-types-globals.gdb/basic-types-globals.debugger.script:9: Error in sourced command file:
No symbol "basic_types_globals::B" in current context.



---- [debuginfo-gdb] src/test/debuginfo/basic-types-globals-lto.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = false
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/i686-unknown-linux-gnu/test/debuginfo/basic-types-globals-lto.gdb/basic-types-globals-lto.debugger.script"
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x6d17: file /checkout/src/test/debuginfo/basic-types-globals-lto.rs, line 75.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_globals_lto::main::hd71d52f40e1ad54c () at /checkout/src/test/debuginfo/basic-types-globals-lto.rs:75
75     _zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/i686-unknown-linux-gnu/test/debuginfo/basic-types-globals-lto.gdb/basic-types-globals-lto.debugger.script:9: Error in sourced command file:
No symbol "basic_types_globals::B" in current context.



failures:
