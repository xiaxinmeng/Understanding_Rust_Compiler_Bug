
---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.8

error: line not found in debugger output: $13 = Some = {"abc"}
status: exit code: 0
command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/gdb-pretty-struct-and-enums.debugger.script
stdout:
------------------------------------------
GNU gdb (GDB) 7.8
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-unknown-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0xc8d: file /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 185.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, gdb_pretty_struct_and_enums::main () at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:185
185     zzz(); // #break
$1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false, the_fourth_field = "I'm so pretty, oh so pretty..."}
$2 = {true, 103, "blub"}
$3 = TupleStruct = {-104.5, 105}
$4 = EmptyStruct
$5 = CStyleEnumVar1
$6 = CStyleEnumVar2
$7 = CStyleEnumVar3
$8 = MixedEnumCStyleVar
$9 = MixedEnumTupleVar = {106, 107, false}
$10 = MixedEnumStructVar = {field1 = 108.5, field2 = 109}
$11 = Some = {110}
$12 = None
$13 = {RUST$ENCODED$ENUM$0$0$None = Some = {"abc"}}
$14 = {RUST$ENCODED$ENUM$0$0$None = Some = {""}}
$15 = NestedVariant1 = {NestedStruct = {regular_struct = RegularStruct = {the_first_field = 111, the_second_field = 112.5, the_third_field = true, the_fourth_field = "NestedStructString1"}, tuple_struct = TupleStruct = {113.5, 114}, empty_struct = EmptyStruct, c_style_enum = CStyleEnumVar2, mixed_enum = MixedEnumTupleVar = {115, 116, false}}}
$16 = NestedVariant2 = {abc = NestedStruct = {regular_struct = RegularStruct = {the_first_field = 117, the_second_field = 118.5, the_third_field = false, the_fourth_field = "NestedStructString10"}, tuple_struct = TupleStruct = {119.5, 120}, empty_struct = EmptyStruct, c_style_enum = CStyleEnumVar3, mixed_enum = MixedEnumStructVar = {field1 = 121.5, field2 = -122}}}
$17 = {RUST$ENCODED$ENUM$0$1$0$0$0$None = Some = {{0, Vec<usize>(len: 0, cap: 0)}}}
$18 = {RUST$ENCODED$ENUM$0$0$0$0$0$None = Some = {""}}
A debugging session is active.

    Inferior 1 [process 7871] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
Can't read symbols from system-supplied DSO at 0x2aaaaaccb000: File truncated
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 
Python Exception <class 'gdb.error'> Cannot convert value to int.: 

------------------------------------------

thread '[debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs' panicked at 'explicit panic', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/compiletest/runtest.rs:1518



failures:
    [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs
