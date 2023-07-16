
$ PATH=/opt/gdb/bin:${PATH} ./x.py test --stage 2
...
running 118 tests
.iiiii..i......i..i...i..i.i..i.iFi.....i..i.....i..........iiii.........i.i..FFiF.i........ii.i.ii. 100/118
.....i.iii......ii
failures:

---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8002000

error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
status: exit code: 0
command: "/opt/gdb/bin/gdb" "-quiet" "-batch" "-nx" "-command=/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7/gdb-pretty-struct-and-enums-pre-gdb-7-7.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 8.2
Copyright (C) 2018 Free Software Foundation, Inc.
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
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0xacf: file /home/vext01/source/rust/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs, line 70.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, gdb_pretty_struct_and_enums_pre_gdb_7_7::main () at /home/vext01/source/rust/src/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs:70
70	    zzz(); // #break
$1 = gdb_pretty_struct_and_enums_pre_gdb_7_7::RegularStruct {the_first_field: 101, the_second_field: 102.5, the_third_field: false}
$2 = gdb_pretty_struct_and_enums_pre_gdb_7_7::EmptyStruct
$3 = gdb_pretty_struct_and_enums_pre_gdb_7_7::CStyleEnum::CStyleEnumVar1
$4 = gdb_pretty_struct_and_enums_pre_gdb_7_7::CStyleEnum::CStyleEnumVar2
$5 = gdb_pretty_struct_and_enums_pre_gdb_7_7::CStyleEnum::CStyleEnumVar3
A debugging session is active.

	Inferior 1 [process 18799] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7/a.
Use `info auto-load python-scripts [REGEXP]' to list them.

------------------------------------------

thread '[debuginfo-both] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [debuginfo-both] debuginfo/pretty-huge-vec.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8002000

error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
status: exit code: 0
command: "/opt/gdb/bin/gdb" "-quiet" "-batch" "-nx" "-command=/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/pretty-huge-vec.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 8.2
Copyright (C) 2018 Free Software Foundation, Inc.
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
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x14ea: file /home/vext01/source/rust/src/test/debuginfo/pretty-huge-vec.rs, line 38.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_huge_vec::main () at /home/vext01/source/rust/src/test/debuginfo/pretty-huge-vec.rs:38
38	    zzz(); // #break
$1 = alloc::vec::Vec<u8> {buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {ptr: core::ptr::Unique<u8> {pointer: core::nonzero::NonZero<*const u8> (0x7fffbb54b010 "\000"), _marker: core::marker::PhantomData<u8>}, cap: 1000000000, a: alloc::alloc::Global}, len: 1000000000}
$2 = &[u8] {data_ptr: 0x7fffbb54b010 "\000", length: 1000000000}
A debugging session is active.

	Inferior 1 [process 19518] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/a.
Use `info auto-load python-scripts [REGEXP]' to list them.

------------------------------------------

thread '[debuginfo-both] debuginfo/pretty-huge-vec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9

---- [debuginfo-both] debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8002000

error: line not found in debugger output: $1 = BTreeSet<i32>(len: 3) = {3, 5, 7}
status: exit code: 0
command: "/opt/gdb/bin/gdb" "-quiet" "-batch" "-nx" "-command=/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 8.2
Copyright (C) 2018 Free Software Foundation, Inc.
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
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x11066: file /home/vext01/source/rust/src/test/debuginfo/pretty-std-collections.rs, line 57.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_std_collections::main () at /home/vext01/source/rust/src/test/debuginfo/pretty-std-collections.rs:57
57	    zzz(); // #break
$1 = alloc::collections::btree::set::BTreeSet<i32> {map: alloc::collections::btree::map::BTreeMap<i32, ()> {root: alloc::collections::btree::node::Root<i32, ()> {node: alloc::collections::btree::node::BoxedNode<i32, ()> {ptr: core::ptr::Unique<alloc::collections::btree::node::LeafNode<i32, ()>> {pointer: core::nonzero::NonZero<*const alloc::collections::btree::node::LeafNode<i32, ()>> (0x555555774130), _marker: core::marker::PhantomData<alloc::collections::btree::node::LeafNode<i32, ()>>}}, height: 0}, length: 3}}
$2 = alloc::collections::btree::map::BTreeMap<i32, i32> {root: alloc::collections::btree::node::Root<i32, i32> {node: alloc::collections::btree::node::BoxedNode<i32, i32> {ptr: core::ptr::Unique<alloc::collections::btree::node::LeafNode<i32, i32>> {pointer: core::nonzero::NonZero<*const alloc::collections::btree::node::LeafNode<i32, i32>> (0x555555774170), _marker: core::marker::PhantomData<alloc::collections::btree::node::LeafNode<i32, i32>>}}, height: 0}, length: 3}
$3 = alloc::collections::vec_deque::VecDeque<i32> {tail: 0, head: 3, buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global> {ptr: core::ptr::Unique<i32> {pointer: core::nonzero::NonZero<*const i32> (0x5555557741e0), _marker: core::marker::PhantomData<i32>}, cap: 8, a: alloc::alloc::Global}}
A debugging session is active.

	Inferior 1 [process 19548] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/a.
Use `info auto-load python-scripts [REGEXP]' to list them.

------------------------------------------

thread '[debuginfo-both] debuginfo/pretty-std-collections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9

---- [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8002000

error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
status: exit code: 0
command: "/opt/gdb/bin/gdb" "-quiet" "-batch" "-nx" "-command=/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 8.2
Copyright (C) 2018 Free Software Foundation, Inc.
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
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1004: file /home/vext01/source/rust/src/test/debuginfo/pretty-uninitialized-vec.rs, line 31.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_uninitialized_vec::main () at /home/vext01/source/rust/src/test/debuginfo/pretty-uninitialized-vec.rs:31
31	    zzz(); // #break
$1 = alloc::vec::Vec<i32> {buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global> {ptr: core::ptr::Unique<i32> {pointer: core::nonzero::NonZero<*const i32> (0x5555557580a0), _marker: core::marker::PhantomData<i32>}, cap: 93824992237456, a: alloc::alloc::Global}, len: 93824992233728}
A debugging session is active.

	Inferior 1 [process 19578] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
warning: Unsupported auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/a.
Use `info auto-load python-scripts [REGEXP]' to list them.

------------------------------------------

thread '[debuginfo-both] debuginfo/pretty-uninitialized-vec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9


failures:
    [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums-pre-gdb-7-7.rs
    [debuginfo-both] debuginfo/pretty-huge-vec.rs
    [debuginfo-both] debuginfo/pretty-std-collections.rs
    [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs

test result: FAILED. 78 passed; 4 failed; 36 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22


command did not execute successfully: "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/home/vext01/source/rust/src/test/debuginfo" "--build-base" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/opt/gdb/bin/gdb" "--quiet" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /home/vext01/source/rust/build/bootstrap/debug/bootstrap test --stage 2
Build completed unsuccessfully in 0:00:14
