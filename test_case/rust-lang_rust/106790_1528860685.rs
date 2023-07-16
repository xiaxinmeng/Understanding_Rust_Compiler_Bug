plain
...............ii..i.i..i.......i........i............F..

failures:

---- [debuginfo-gdb] tests/debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: $10 = VecDeque(size=3) = {5, 3, 7}
status: exit status: 0
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
GNU gdb (Ubuntu 12.1-0ubuntu1~22.04) 12.1
Copyright (C) 2022 Free Software Foundation, Inc.
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x4f993: file /checkout/tests/debuginfo/pretty-std-collections.rs, line 158.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_std_collections::main () at /checkout/tests/debuginfo/pretty-std-collections.rs:158
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
$10 = alloc::collections::vec_deque::VecDeque<i32, alloc::alloc::Global> {head: 0, len: 3, buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global> {ptr: core::ptr::unique::Unique<i32> {pointer: core::ptr::non_null::NonNull<i32> {pointer: 0x5555555df020}, _marker: core::marker::PhantomData<i32>}, cap: alloc::raw_vec::Cap (4), alloc: alloc::alloc::Global}}
$11 = alloc::collections::vec_deque::VecDeque<i32, alloc::alloc::Global> {head: 1, len: 7, buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global> {ptr: core::ptr::unique::Unique<i32> {pointer: core::ptr::non_null::NonNull<i32> {pointer: 0x5555555df040}, _marker: core::marker::PhantomData<i32>}, cap: alloc::raw_vec::Cap (8), alloc: alloc::alloc::Global}}
$12 = HashMap(size=4) = {[1] = 10, [2] = 20, [3] = 30, [4] = 40}
$13 = HashSet(size=4) = {1, 2, 3, 4}


 Inferior 1 [process 150520] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'>: Cannot convert value to long.
Python Exception <class 'gdb.error'>: Cannot convert value to long.
Python Exception <class 'gdb.error'>: Cannot convert value to long.
Python Exception <class 'gdb.error'>: Cannot convert value to long.



failures:
