plain
 finished in 12.358 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 135 tests
iiiiiii.i..i.ii.....i.iiii...............i.F.........i.iiii...ii.......ii......i........ 88/135
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....F.iiF.i.i.........i.......i.............F.

---- [debuginfo-gdb] src/test/debuginfo/empty-string.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/empty-string.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 9002000

error: line not found in debugger output: $1 = ""
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string.gdb/empty-string.debugger.script"
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x14b1: file /checkout/src/test/debuginfo/empty-string.rs, line 33.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, empty_string::main () at /checkout/src/test/debuginfo/empty-string.rs:33
33     zzz(); // #break
$1 = alloc::string::String {vec: alloc::vec::Vec<u8, alloc::alloc::Global> {buf: alloc::boxed::Box<[core::mem::maybe_uninit::MaybeUninit<u8>], alloc::alloc::Global> {data_ptr: 0x1, length: 0}, phantom: core::marker::PhantomData<u8>, len: 0}}
$2 = ""
A debugging session is active.

 Inferior 1 [process 121773] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'> There is no member named ptr.: 
Python Exception <class 'gdb.error'> There is no member named ptr.: 
Python Exception <class 'gdb.error'> There is no member named 0.:


---- [debuginfo-gdb] src/test/debuginfo/pretty-huge-vec.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/pretty-huge-vec.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 9002000

error: line not found in debugger output: $1 = Vec(size=1000000000) = {[...]...}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec.gdb/pretty-huge-vec.debugger.script"
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x2b25: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 27.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_huge_vec::main () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:27
27     zzz(); // #break
$1 = alloc::vec::Vec<u8, alloc::alloc::Global> {buf: alloc::boxed::Box<[core::mem::maybe_uninit::MaybeUninit<u8>], alloc::alloc::Global> {data_ptr: 0x7fffbc23e010, length: 1000000000}, phantom: core::marker::PhantomData<u8>, len: 1000000000}
$2 = &[u8](size=1000000000) = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0...}
A debugging session is active.

 Inferior 1 [process 122780] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'> There is no member named 0.: 
Python Exception <class 'gdb.error'> There is no member named 0.:


---- [debuginfo-gdb] src/test/debuginfo/pretty-uninitialized-vec.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/pretty-uninitialized-vec.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 9002000

error: line not found in debugger output: $1 = Vec(size=[...])[...]
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec.gdb/pretty-uninitialized-vec.debugger.script"
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x2204: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_uninitialized_vec::main () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
21     zzz(); // #break
$1 = alloc::vec::Vec<i32, alloc::alloc::Global> {buf: alloc::boxed::Box<[core::mem::maybe_uninit::MaybeUninit<i32>], alloc::alloc::Global> {data_ptr: 0x7, length: 34359738375}, phantom: core::marker::PhantomData<i32>, len: 140737353938304}
A debugging session is active.

 Inferior 1 [process 122797] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'> There is no member named 0.: 
Python Exception <class 'gdb.error'> There is no member named 0.:


---- [debuginfo-gdb] src/test/debuginfo/pretty-std-collections.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 9002000

error: line not found in debugger output: $10 = VecDeque(size=3) = {5, 3, 7}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x50a18: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 158.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:158
158     zzz(); // #break
$1 = BTreeSet(size=15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
$2 = BTreeSet(size=0)
$3 = BTreeMap(size=15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}
$4 = BTreeMap(size=0)
$5 = BTreeMap(size=2) = {[false] = core::option::Option<bool>::None, [true] = core::option::Option<bool>::Some(true)}
$6 = BTreeMap(size=15) = {[0] = pretty_std_collections::MyLeafNode (0), [1] = pretty_std_collections::MyLeafNode (1), [2] = pretty_std_collections::MyLeafNode (2), [3] = pretty_std_collections::MyLeafNode (3), [4] = pretty_std_collections::MyLeafNode (4), [5] = pretty_std_collections::MyLeafNode (5), [6] = pretty_std_collections::MyLeafNode (6), [7] = pretty_std_collections::MyLeafNode (7), [8] = pretty_std_collections::MyLeafNode (8), [9] = pretty_std_collections::MyLeafNode (9), [10] = pretty_std_collections::MyLeafNode (10), [11] = pretty_std_collections::MyLeafNode (11), [12] = pretty_std_collections::MyLeafNode (12), [13] = pretty_std_collections::MyLeafNode (13), [14] = pretty_std_collections::MyLeafNode (14)}
$7 = BTreeMap(size=1) = {[()] = 1}
$8 = BTreeMap(size=1) = {[1] = ()}
$9 = BTreeMap(size=1) = {[()] = ()}
$10 = alloc::collections::vec_deque::VecDeque<i32, alloc::alloc::Global> {tail: 0, head: 3, buf: alloc::boxed::Box<[core::mem::maybe_uninit::MaybeUninit<i32>], alloc::alloc::Global> {data_ptr: 0x5555555d6ff0, length: 8}, phantom: core::marker::PhantomData<i32>}
$11 = alloc::collections::vec_deque::VecDeque<i32, alloc::alloc::Global> {tail: 1, head: 0, buf: alloc::boxed::Box<[core::mem::maybe_uninit::MaybeUninit<i32>], alloc::alloc::Global> {data_ptr: 0x5555555d7020, length: 8}, phantom: core::marker::PhantomData<i32>}
$12 = HashMap(size=4) = {[1] = 10, [2] = 20, [3] = 30, [4] = 40}
$13 = HashSet(size=4) = {1, 2, 3, 4}
A debugging session is active.

 Inferior 1 [process 123467] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'> There is no member named len.: 
Python Exception <class 'gdb.error'> There is no member named len.: 
Python Exception <class 'gdb.error'> There is no member named len.: 
Python Exception <class 'gdb.error'> There is no member named len.:



failures:
