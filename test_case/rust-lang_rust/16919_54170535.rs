

---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.7

    error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}

    status: exit code: 0
    command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/gdb-pretty-struct-and-enums-pre-gdb-7-7.debugger.script
    stdout:
    ------------------------------------------
    GNU gdb (GDB) 7.7
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
    Breakpoint 1 at 0x940: file /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs, line 75.
    static void gdb-pretty-struct-and-enums-pre-gdb-7-7::zzz(void);
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib64/libthread_db.so.1".

    Breakpoint 1, gdb-pretty-struct-and-enums-pre-gdb-7-7::zzz () at /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs:75
    75  fn zzz() { () }
    0x00007ffff7ffe8d7 in gdb-pretty-struct-and-enums-pre-gdb-7-7::main () at /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs:72
    72      zzz();
    $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
    $2 = {<No data fields>}
    $3 = CStyleEnumVar1
    $4 = CStyleEnumVar2
    $5 = CStyleEnumVar3
    A debugging session is active.

        Inferior 1 [process 30775] will be killed.

    Quit anyway? (y or n) [answered Y; input not from terminal]

    ------------------------------------------
    stderr:
    ------------------------------------------

    ------------------------------------------

    task '[debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs' failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/compiletest/runtest.rs:1415


---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.7

    error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false, the_fourth_field = "I'm so pretty, oh so pretty..."}

    status: exit code: 0
    command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/gdb-pretty-struct-and-enums.debugger.script
    stdout:
    ------------------------------------------
    GNU gdb (GDB) 7.7
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
    Breakpoint 1 at 0xb40: file /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 172.
    static void gdb-pretty-struct-and-enums::zzz(void);
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib64/libthread_db.so.1".

    Breakpoint 1, gdb-pretty-struct-and-enums::zzz () at /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:172
    172 fn zzz() { () }
    0x00007ffff7ffead6 in gdb-pretty-struct-and-enums::main () at /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:169
    169     zzz();
    $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false, the_fourth_field = {data_ptr = 0x7ffff7ffecb0 <str1128> "I'm so pretty, oh so pretty...blub", length = 30}}
    $2 = {true, 103, {data_ptr = 0x7ffff7ffecce <str1129> "blub", length = 4}}
    $3 = {-104.5, 105}
    $4 = {<No data fields>}
    $5 = CStyleEnumVar1
    $6 = CStyleEnumVar2
    $7 = CStyleEnumVar3
    $8 = {{MixedEnumCStyleVar}, {MixedEnumCStyleVar, 0, 58128, 255}, {MixedEnumCStyleVar, field1 = 6.9533558074690005e-310, field2 = 1}}
    $9 = {{MixedEnumTupleVar}, {MixedEnumTupleVar, 106, 107, false}, {MixedEnumTupleVar, field1 = 6.9533483476988382e-310, field2 = 0}}
    $10 = {{MixedEnumStructVar}, {MixedEnumStructVar, 32767, 0, false}, {MixedEnumStructVar, field1 = 108.5, field2 = 109}}
    $11 = {{Some}, {Some, 110}}
    $12 = {{None}, {None, 140733193388033}}
    $13 = {{NestedVariant1, {regular_struct = {the_first_field = 111, the_second_field = 112.5, the_third_field = true, the_fourth_field = {data_ptr = 0x7ffff7ffece0 <str1131> "NestedStructString1", length = 19}}, tuple_struct = {113.5, 114}, empty_struct = {<No data fields>}, c_style_enum = CStyleEnumVar2, mixed_enum = {{MixedEnumTupleVar}, {MixedEnumTupleVar, 115, 116, false}, {MixedEnumTupleVar, field1 = 6.9533483476992829e-310, field2 = -7496}}}}, {NestedVariant1, abc = {regular_struct = {the_first_field = 111, the_second_field = 112.5, the_third_field = true, the_fourth_field = {data_ptr = 0x7ffff7ffece0 <str1131> "NestedStructString1", length = 19}}, tuple_struct = {113.5, 114}, empty_struct = {<No data fields>}, c_style_enum = CStyleEnumVar2, mixed_enum = {{MixedEnumTupleVar}, {MixedEnumTupleVar, 115, 116, false}, {MixedEnumTupleVar, field1 = 6.9533483476992829e-310, field2 = -7496}}}}}
    $14 = {{NestedVariant2, {regular_struct = {the_first_field = 117, the_second_field = 118.5, the_third_field = false, the_fourth_field = {data_ptr = 0x7ffff7ffed00 <str1132> "NestedStructString10\001\033\003;8", length = 20}}, tuple_struct = {119.5, 120}, empty_struct = {<No data fields>}, c_style_enum = CStyleEnumVar3, mixed_enum = {{MixedEnumStructVar}, {MixedEnumStructVar, 0, 0, false}, {MixedEnumStructVar, field1 = 121.5, field2 = -122}}}}, {NestedVariant2, abc = {regular_struct = {the_first_field = 117, the_second_field = 118.5, the_third_field = false, the_fourth_field = {data_ptr = 0x7ffff7ffed00 <str1132> "NestedStructString10\001\033\003;8", length = 20}}, tuple_struct = {119.5, 120}, empty_struct = {<No data fields>}, c_style_enum = CStyleEnumVar3, mixed_enum = {{MixedEnumStructVar}, {MixedEnumStructVar, 0, 0, false}, {MixedEnumStructVar, field1 = 121.5, field2 = -122}}}}}
    A debugging session is active.

        Inferior 1 [process 30788] will be killed.

    Quit anyway? (y or n) [answered Y; input not from terminal]

    ------------------------------------------
    stderr:
    ------------------------------------------

    ------------------------------------------

    task '[debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs' failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/obj/tmp/distcheck/rust-nightly/src/compiletest/runtest.rs:1415



failures:
    [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs
    [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs

test result: FAILED. 91 passed; 2 failed; 5 ignored; 0 measured
