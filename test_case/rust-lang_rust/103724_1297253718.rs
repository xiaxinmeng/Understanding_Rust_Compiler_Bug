plain
 finished in 15.313 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 145 tests
iiiiii.......i..i.ii......i...i.........F......i............iiiii....ii.........i......i 88/145
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......F.......Fii..i.i..i.......i........i............F..

---- [debuginfo-gdb] src/test/debuginfo/embedded-visualizer.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/embedded-visualizer.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: $4 = "Person A" is 10 years old.
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/embedded-visualizer.gdb/embedded-visualizer.debugger.script"
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
Breakpoint 1 at 0x2049: file /checkout/src/test/debuginfo/embedded-visualizer.rs, line 107.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, embedded_visualizer::main () at /checkout/src/test/debuginfo/embedded-visualizer.rs:107
107     zzz(); // #break
Loaded  Script                                                                 
Yes     gdb_load_rust_pretty_printers.py                                       
 full name: /checkout/src/etc/gdb_load_rust_pretty_printers.py
Yes     pretty-printer-embedded_visualizer-0                                   
Yes     pretty-printer-embedded_visualizer-1                                   
Yes     pretty-printer-embedded_visualizer-2                                   
$1 = (0, 0)
$2 = (5, 8)
$3 = ((0, 0), (5, 8))
$4 = "\x55555555cba0\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\n\000\000\000w\000\000\000\x55555555cba0\b\000\000\000\000\000\000\000\b", '\000' <repeats 15 times> is 10 years old.


 Inferior 1 [process 128065] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/numeric-types.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/numeric-types.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: [...]$1 = 11
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/numeric-types.gdb/numeric-types.debugger.script"
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
Breakpoint 1 at 0x19e0: file /checkout/src/test/debuginfo/numeric-types.rs, line 288.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
Breakpoint 1, numeric_types::main () at /checkout/src/test/debuginfo/numeric-types.rs:288
Breakpoint 1, numeric_types::main () at /checkout/src/test/debuginfo/numeric-types.rs:288
288     zzz(); // #break
$1 = core::num::ranged::Ranged<i8, {CONST#968ba93192a620a9}> (11)
$2 = core::num::ranged::Ranged<i16, {CONST#9ba16555371e6509}> (22)
$3 = core::num::ranged::Ranged<i32, {CONST#ba4a3876be0f95bf}> (33)
$4 = core::num::ranged::Ranged<i64, {CONST#ba021daa597cceb6}> (44)
$5 = core::num::ranged::Ranged<i128, {CONST#a520175af2f9bd55}> (55)
$6 = core::num::ranged::Ranged<isize, {CONST#ba021daa597cceb6}> (66)
$7 = core::num::ranged::Ranged<u8, {CONST#968ba93192a620a9}> (77)
$8 = core::num::ranged::Ranged<u16, {CONST#9ba16555371e6509}> (88)
$9 = core::num::ranged::Ranged<u32, {CONST#ba4a3876be0f95bf}> (99)
$10 = core::num::ranged::Ranged<u64, {CONST#ba021daa597cceb6}> (100)
$11 = core::num::ranged::Ranged<u128, {CONST#a520175af2f9bd55}> (111)
$12 = core::num::ranged::Ranged<usize, {CONST#ba021daa597cceb6}> (122)


 Inferior 1 [process 129718] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/pretty-huge-vec.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/pretty-huge-vec.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: $1 = Vec(size=1000000000) = {[...]...}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec.gdb/pretty-huge-vec.debugger.script"
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
Breakpoint 1 at 0x1e8f: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 27.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_huge_vec::main () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:27
27     zzz(); // #break
$1 = Vec(size=1000000000)
$2 = &[u8](size=1000000000) = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0...}


 Inferior 1 [process 129855] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'>: Structure has no component named operator+.


---- [debuginfo-gdb] src/test/debuginfo/pretty-std-collections.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: $10 = VecDeque(size=3) = {5, 3, 7}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
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
Breakpoint 1 at 0x57a43: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 158.
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
$10 = VecDeque(size=3)
$11 = VecDeque(size=7)
$12 = std::collections::hash::map::HashMap<u64, u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {base: hashbrown::map::HashMap<u64, u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>, alloc::alloc::Global> {hash_builder: core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher> (core::marker::PhantomData<fn() -> pretty_std_collections::SimpleHasher>), table: hashbrown::raw::RawTable<(u64, u64), alloc::alloc::Global> {table: hashbrown::raw::RawTableInner<alloc::alloc::Global> {bucket_mask: 7, ctrl: core::ptr::non_null::NonNull<u8> {pointer: core::num::ranged::Ranged<*const u8, {CONST#ba021daa597cceb6}> (0x5555555ef130)}, growth_left: 3, items: 4, alloc: alloc::alloc::Global}, marker: core::marker::PhantomData<(u64, u64)>}}}
$13 = std::collections::hash::set::HashSet<u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>> {base: hashbrown::set::HashSet<u64, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>, alloc::alloc::Global> {map: hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher>, alloc::alloc::Global> {hash_builder: core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher> (core::marker::PhantomData<fn() -> pretty_std_collections::SimpleHasher>), table: hashbrown::raw::RawTable<(u64, ()), alloc::alloc::Global> {table: hashbrown::raw::RawTableInner<alloc::alloc::Global> {bucket_mask: 7, ctrl: core::ptr::non_null::NonNull<u8> {pointer: core::num::ranged::Ranged<*const u8, {CONST#ba021daa597cceb6}> (0x5555555ef090)}, growth_left: 3, items: 4, alloc: alloc::alloc::Global}, marker: core::marker::PhantomData<(u64, ())>}}}}


 Inferior 1 [process 131057] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
Python Exception <class 'gdb.error'>: Structure has no component named operator+.
Python Exception <class 'gdb.error'>: Structure has no component named operator+.
Python Exception <class 'gdb.error'>: Structure has no component named operator+.
Python Exception <class 'gdb.error'>: Structure has no component named operator+.
Python Exception <class 'gdb.error'>: Structure has no component named operator+.
Python Exception <class 'gdb.error'>: Structure has no component named operator+.



failures:
