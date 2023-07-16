plain
.................................................................................................... 9500/11877
.................................................................................................... 9600/11877
..............................................................................i......i.............. 9700/11877
.................................................................................................... 9800/11877
........................iiiiiii..iiiiiii............................................................ 9900/11877
.................................................................................................... 10100/11877
.................................................................................................... 10200/11877
.................................................................................................... 10300/11877
.................................................................................................... 10400/11877
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out; finished in 0.13s

 finished in 0.200 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
ii....i..i.iFi...F....ii....i.i....ii.............................i.F.i.....F......ii.....i..i.....F 100/116
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [debuginfo-gdb] debuginfo/by-value-non-immediate-argument.rs stdout ----
---- [debuginfo-gdb] debuginfo/by-value-non-immediate-argument.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001001

error: line not found in debugger output: $7 = by_value_non_immediate_argument::Enum::Case1{x: 0, y: 8970181431921507452}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/by-value-non-immediate-argument.gdb/by-value-non-immediate-argument.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 8.1.1-0ubuntu1) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
Copyright (C) 2018 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x82f: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 84.
Breakpoint 2 at 0x86b: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 88.
Breakpoint 3 at 0x885: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 92.
Breakpoint 4 at 0x895: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 98.
Breakpoint 5 at 0x8a5: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 110.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, by_value_non_immediate_argument::fun (s=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:84
84     zzz(); // #break
$1 = by_value_non_immediate_argument::Struct {a: 1, b: 2.5}

Breakpoint 2, by_value_non_immediate_argument::fun_fun () at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:88
88     zzz(); // #break
$2 = by_value_non_immediate_argument::Struct {a: 3, b: 4.5}
$4 = 6.5


Breakpoint 3, by_value_non_immediate_argument::tup (a=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:92
92     zzz(); // #break
$5 = (7, 8, 9.5, 10.5)

Breakpoint 4, by_value_non_immediate_argument::new_type (a=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:98
98     zzz(); // #break
$6 = by_value_non_immediate_argument::Newtype (11.5, 12.5, 13, 14)

Breakpoint 5, by_value_non_immediate_argument::by_val_enum (x=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:110
110     zzz(); // #break
$7 = by_value_non_immediate_argument::Enum
[Inferior 1 (process 16355) exited normally]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-gdb] debuginfo/basic-types-metadata.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001001

error: line not found in debugger output: type = *mut fn ()
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-metadata.gdb/basic-types-metadata.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 8.1.1-0ubuntu1) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
Copyright (C) 2018 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x135e: file /checkout/src/test/debuginfo/basic-types-metadata.rs, line 89.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_metadata::main () at /checkout/src/test/debuginfo/basic-types-metadata.rs:89
89     _zzz(); // #break
type = ()
type = isize
type = char
type = i8
type = i16
---
type = u32
type = u64
type = f32
type = f64
type = void (*)(void)
All functions matching regular expression "_yyy":
File /checkout/src/test/debuginfo/basic-types-metadata.rs:
File /checkout/src/test/debuginfo/basic-types-metadata.rs:
static fn basic_types_metadata::_yyy();
type = struct basic_types_metadata::main::closure-0
type = struct basic_types_metadata::main::closure-1 (
  bool *,
type = struct basic_types_metadata::main::closure-2 (
  bool *,
  isize *,
)
)
[Inferior 1 (process 16381) exited with code 0145]
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/debuginfo/basic-types-metadata.rs:94:17
thread 'main' panicked at 'explicit panic', /checkout/src/test/debuginfo/basic-types-metadata.rs:94:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

------------------------------------------


---- [debuginfo-gdb] debuginfo/method-on-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001001

error: line not found in debugger output: $1 = method_on_enum::Enum::Variant2(117901063)
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/method-on-enum.gdb/method-on-enum.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 8.1.1-0ubuntu1) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
Copyright (C) 2018 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x136d: file /checkout/src/test/debuginfo/method-on-enum.rs, line 123.
Breakpoint 2 at 0x13e7: file /checkout/src/test/debuginfo/method-on-enum.rs, line 128.
Breakpoint 3 at 0x144d: file /checkout/src/test/debuginfo/method-on-enum.rs, line 133.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, method_on_enum::Enum::self_by_ref (self=0x7fffffffe130, arg1=-1, arg2=-2) at /checkout/src/test/debuginfo/method-on-enum.rs:123
123         zzz(); // #break
$1 = method_on_enum::Enum
$2 = -1
$3 = -2

Breakpoint 2, method_on_enum::Enum::self_by_val (self=..., arg1=-3, arg2=-4) at /checkout/src/test/debuginfo/method-on-enum.rs:128
128         zzz(); // #break
$4 = method_on_enum::Enum
$5 = -3
$6 = -4

Breakpoint 1, method_on_enum::Enum::self_by_ref (self=0x555555758ae0, arg1=-5, arg2=-6) at /checkout/src/test/debuginfo/method-on-enum.rs:123
123         zzz(); // #break
$7 = method_on_enum::Enum
$8 = -5
$9 = -6

Breakpoint 2, method_on_enum::Enum::self_by_val (self=..., arg1=-7, arg2=-8) at /checkout/src/test/debuginfo/method-on-enum.rs:128
128         zzz(); // #break
$10 = method_on_enum::Enum
$11 = -7
$12 = -8

Breakpoint 3, method_on_enum::Enum::self_owned (self=0x555555758ae0, arg1=-9, arg2=-10) at /checkout/src/test/debuginfo/method-on-enum.rs:133
133         zzz(); // #break
$13 = method_on_enum::Enum
$14 = -9
$15 = -10
[Inferior 1 (process 17451) exited normally]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-gdb] debuginfo/option-like-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001001

error: line not found in debugger output: $1 = core::option::Option<&u32>::Some(0x12345678)
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/option-like-enum.gdb/option-like-enum.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 8.1.1-0ubuntu1) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
Copyright (C) 2018 Free Software Foundation, Inc.
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
Breakpoint 1 at 0xac2: file /checkout/src/test/debuginfo/option-like-enum.rs, line 162.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, option_like_enum::main () at /checkout/src/test/debuginfo/option-like-enum.rs:162
162     zzz(); // #break
$1 = core::option::Option<&u32>
$2 = core::option::Option<&u32>
$3 = option_like_enum::MoreFields
$4 = (isize *) 0x1
$5 = option_like_enum::NamedFields
$6 = (isize *) 0x0
$7 = option_like_enum::NestedNonZero
$8 = option_like_enum::NestedNonZero
[Inferior 1 (process 17711) exited normally]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-gdb] debuginfo/struct-in-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001001

error: line not found in debugger output: $1 = struct_in_enum::Regular::Case1(0, struct_in_enum::Struct {x: 2088533116, y: 2088533116, z: 31868})
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/struct-in-enum.gdb/struct-in-enum.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 8.1.1-0ubuntu1) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
Copyright (C) 2018 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x93c: file /checkout/src/test/debuginfo/struct-in-enum.rs, line 82.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, struct_in_enum::main () at /checkout/src/test/debuginfo/struct-in-enum.rs:82
82     zzz(); // #break
$1 = struct_in_enum::Regular
$2 = struct_in_enum::Regular
$3 = struct_in_enum::Univariant
A debugging session is active.

 Inferior 1 [process 18096] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------

---
test result: FAILED. 93 passed; 5 failed; 18 ignored; 0 measured; 0 filtered out; finished in 4.48s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:37
