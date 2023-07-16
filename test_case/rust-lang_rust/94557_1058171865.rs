plain

Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
failures:

---- [debuginfo-gdb] debuginfo/c-style-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = TheOnlyVariant
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/c-style-enum.gdb/c-style-enum.debugger.script"
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
Breakpoint 1 at 0xa02: file /checkout/src/test/debuginfo/c-style-enum.rs, line 185.
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/c-style-enum.gdb/c-style-enum.debugger.script:8: Error in sourced command file:
No symbol "c_style_enum::SINGLE_VARIANT" in current context.


---- [debuginfo-gdb] debuginfo/basic-types-globals-metadata.rs stdout ----
---- [debuginfo-gdb] debuginfo/basic-types-globals-metadata.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001
error: line not found in debugger output: type = bool
status: exit status: 0
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals-metadata.gdb/basic-types-globals-metadata.debugger.script"
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
Breakpoint 1 at 0x747: file /checkout/src/test/debuginfo/basic-types-globals-metadata.rs, line 71.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_globals_metadata::main::h40cb61f9869a7dee () at /checkout/src/test/debuginfo/basic-types-globals-metadata.rs:71
71     _zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals-metadata.gdb/basic-types-globals-metadata.debugger.script:9: Error in sourced command file:
No symbol "basic_types_globals_metadata::B" in current context.


---- [debuginfo-gdb] debuginfo/function-arg-initialization.rs stdout ----
---- [debuginfo-gdb] debuginfo/function-arg-initialization.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $4 = {a = 3, b = 4, c = 5, d = 6, e = 7, f = 8, g = 9, h = 10}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/function-arg-initialization.gdb/function-arg-initialization.debugger.script"
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
Breakpoint 1 at 0x9a4: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 224.
Breakpoint 2 at 0x9b4: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 239.
Breakpoint 3 at 0x9fb: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 243.
Breakpoint 4 at 0xa7b: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 248.
Breakpoint 5 at 0xaf6: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 253.
Breakpoint 6 at 0xb17: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 257.
Breakpoint 7 at 0xb37: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 261.
Breakpoint 8 at 0xb6b: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 265.
Breakpoint 9 at 0xbfb: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 269.
Breakpoint 10 at 0xcc5: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 277.
Breakpoint 11 at 0xdd5: file /checkout/src/test/debuginfo/function-arg-initialization.rs, line 285.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, function_arg_initialization::immediate_args::hbf5a305ddaf18190 (a=1, b=true, c=2.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:224
224     zzz(); // #break
$2 = true
$3 = 2.5


Breakpoint 2, function_arg_initialization::non_immediate_args::hf43956f1f30a5e75 (a=..., b=...) at /checkout/src/test/debuginfo/function-arg-initialization.rs:239
239     zzz(); // #break
$4 = <optimized out>
$5 = <optimized out>

Breakpoint 3, function_arg_initialization::binding::h580db45536e07170 (a=19, b=20, c=21.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:243
243     let x = 0; // #break
$7 = 20
$8 = 21.5



Breakpoint 4, function_arg_initialization::assignment::h87aa6ccbd05d7cc5 (a=22, b=23, c=24.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:248
248     a = b; // #break
$10 = 23
$11 = 24.5



Breakpoint 5, function_arg_initialization::function_call::h43e92a245bec8bab (x=25, y=26, z=27.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:253
253     zzz(); // #break
$13 = 26
$14 = 27.5


Breakpoint 6, function_arg_initialization::identifier::he20a9c69f42423fb (x=28, y=29, z=30.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:258
$15 = 28
$16 = 29
$17 = 30.5


Breakpoint 7, function_arg_initialization::return_expr::h78dc8ba07747f0a1 (x=31, y=32, z=33.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:262
$18 = 31
$19 = 32
$20 = 33.5


Breakpoint 8, function_arg_initialization::arithmetic_expr::hfd41823cb9fd0efc (x=34, y=35, z=36.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:265
265     x + y // #break
$22 = 35
$23 = 36.5


Breakpoint 9, function_arg_initialization::if_expr::hbd519300d6982db7 (x=37, y=38, z=39.5) at /checkout/src/test/debuginfo/function-arg-initialization.rs:269
269     if x + y < 1000 { // #break
$25 = 38
$26 = 39.5


Breakpoint 10, function_arg_initialization::while_expr::h540f3233b02d76c0 (x=40, y=41, z=42) at /checkout/src/test/debuginfo/function-arg-initialization.rs:277
277     while x + y > 1000 { // #break
$28 = 41
$29 = 42


Breakpoint 11, function_arg_initialization::loop_expr::h796e4813a65992d6 (x=43, y=44, z=45) at /checkout/src/test/debuginfo/function-arg-initialization.rs:285
285         x += z; // #break
$31 = 44
$32 = 45


Breakpoint 11, function_arg_initialization::loop_expr::h796e4813a65992d6 (x=88, y=44, z=45) at /checkout/src/test/debuginfo/function-arg-initialization.rs:285
285         x += z; // #break
A debugging session is active.

 Inferior 1 [process 242497] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none


---- [debuginfo-gdb] debuginfo/lexical-scopes-in-block-expression.rs stdout ----
---- [debuginfo-gdb] debuginfo/lexical-scopes-in-block-expression.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001
error: line not found in debugger output: $1 = 0
status: exit status: 0
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression.gdb/lexical-scopes-in-block-expression.debugger.script"
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
Breakpoint 1 at 0x98d: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 420.
Breakpoint 2 at 0x9c5: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 426.
Breakpoint 3 at 0xa0e: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 434.
Breakpoint 4 at 0xa20: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 439.
Breakpoint 5 at 0xa5a: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 445.
Breakpoint 6 at 0xaa3: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 451.
Breakpoint 7 at 0xab5: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 457.
Breakpoint 8 at 0xaef: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 463.
Breakpoint 9 at 0xb28: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 469.
Breakpoint 10 at 0xb3a: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 474.
Breakpoint 11 at 0xb74: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 480.
Breakpoint 12 at 0xbad: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 486.
Breakpoint 13 at 0xbbf: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 491.
Breakpoint 14 at 0xbf9: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 497.
Breakpoint 15 at 0xc32: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 503.
Breakpoint 16 at 0xc4c: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 509.
Breakpoint 17 at 0xc86: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 515.
Breakpoint 18 at 0xcc7: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 521.
Breakpoint 19 at 0xcd9: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 526.
Breakpoint 20 at 0xd13: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 532.
Breakpoint 21 at 0xddf: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 538.
Breakpoint 22 at 0xe4e: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 544.
Breakpoint 23 at 0xe88: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 550.
Breakpoint 24 at 0xed1: file /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs, line 556.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, lexical_scopes_in_block_expression::main::hccd929e67a05d7d7 () at /checkout/src/test/debuginfo/lexical-scopes-in-block-expression.rs:420
420             zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/lexical-scopes-in-block-expression.gdb/lexical-scopes-in-block-expression.debugger.script:32: Error in sourced command file:
No symbol "lexical_scopes_in_block_expression::MUT_INT" in current context.


---- [debuginfo-gdb] debuginfo/limited-debuginfo.rs stdout ----
---- [debuginfo-gdb] debuginfo/limited-debuginfo.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: [...]void[...]main([...]);
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/limited-debuginfo.gdb/limited-debuginfo.debugger.script"
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
Breakpoint 1 at 0xc98: file /checkout/src/test/debuginfo/limited-debuginfo.rs, line 46.
All defined functions:
File /checkout/src/test/debuginfo/limited-debuginfo.rs:
File /checkout/src/test/debuginfo/limited-debuginfo.rs:
static void limited_debuginfo::main::h3aacd493f3f654f5(void);
static void limited_debuginfo::some_function::hf4db4218f0777f32(void);
static void limited_debuginfo::some_other_function::hd284ec60f34bf8bb(void);
static void limited_debuginfo::zzz::h49b22b741fc7d636(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/clone.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/clone.rs:
static void core::clone::impls::_$LT$impl$u20$core..clone..Clone$u20$for$u20$i32$GT$::clone::h0564888f2e319877(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/cmp.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/cmp.rs:
static void core::cmp::impls::_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$i32$GT$::lt::hfdd1bb7e34af2c20(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/hint.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/hint.rs:
static void core::hint::black_box::h9d01748030dd9572(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/iter/range.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/iter/range.rs:
static void _$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$::spec_next::h16211f77e3005190(void);
static void _$LT$i32$u20$as$u20$core..iter..range..Step$GT$::forward_unchecked::h64781234b45ae1bd(void);
static void core::iter::range::_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$::next::h47d85d5e21b75e42(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/iter/traits/collect.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/iter/traits/collect.rs:
static void _$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$::into_iter::hed1d73fe15b9073b(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/mem/mod.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/mem/mod.rs:
static void core::mem::replace::h4ea2832fc2295132(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/ops/function.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/ops/function.rs:
static void core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h2b77b1527cb3e00f(void);
static void core::ops::function::FnOnce::call_once::h56ff11ed108926a2(void);
static void core::ops::function::FnOnce::call_once::h5faa521bfea15dcb(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/ptr/mod.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/core/src/ptr/mod.rs:
static void core::ptr::drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$::haeb508c53eb62b18(void);
static void core::ptr::read::hba910584d249864f(void);
static void core::ptr::write::hf254166387f51182(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/process.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/process.rs:
static void _$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$::report::h515578618fc50e3b(void);
static void _$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$::report::hfb4c0839435c7dd8(void);
static void std::process::ExitCode::to_i32::h1867b908c9d43b64(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/rt.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/rt.rs:
static void std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc594aa596f7a9f73(void);
static void std::rt::lang_start::he41749aad580f969(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/sys/unix/process/process_common.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/sys/unix/process/process_common.rs:
static void std::sys::unix::process::process_common::ExitCode::as_i32::hc92e22e72fe7e7bf(void);
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/sys_common/backtrace.rs:
File /rustc/377ca6e2090c8dc36ec362e9290d23a287b4f0f5/library/std/src/sys_common/backtrace.rs:
static void std::sys_common::backtrace::__rust_begin_short_backtrace::h8b9d35490786d580(void);
Non-debugging symbols:
0x00000550  _init
0x000005c0  _start
0x000005c0  _start
0x00000600  __x86.get_pc_thunk.bx
0x00000610  deregister_tm_clones
0x00000650  register_tm_clones
0x000006a0  __do_global_dtors_aux
0x000006f0  frame_dummy
0x0000072c  __x86.get_pc_thunk.dx
0x00000cb0  main
0x00000cf0  __libc_csu_init
0x00000d50  __libc_csu_fini
0x00000d54  _fini
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, limited_debuginfo::some_function::hf4db4218f0777f32 () at /checkout/src/test/debuginfo/limited-debuginfo.rs:46
46         zzz(); // #break
No locals.
[Inferior 1 (process 243070) exited normally]
stderr: none


---- [debuginfo-gdb] debuginfo/method-on-enum.rs stdout ----
---- [debuginfo-gdb] debuginfo/method-on-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = {{RUST$ENUM$DISR = Variant2, [...]}, {RUST$ENUM$DISR = Variant2, __0 = 117901063}}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/method-on-enum.gdb/method-on-enum.debugger.script"
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
Breakpoint 1 at 0x13e8: file /checkout/src/test/debuginfo/method-on-enum.rs, line 122.
Breakpoint 2 at 0x146c: file /checkout/src/test/debuginfo/method-on-enum.rs, line 127.
Breakpoint 3 at 0x14e8: file /checkout/src/test/debuginfo/method-on-enum.rs, line 132.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, method_on_enum::Enum::self_by_ref::h8e77efffda08da35 (self=0xffffd168, arg1=-1, arg2=-2) at /checkout/src/test/debuginfo/method-on-enum.rs:122
122         zzz(); // #break
$1 = {<No data fields>}
$2 = -1
$3 = -2

Breakpoint 2, method_on_enum::Enum::self_by_val::h153e020c0168d790 (self=..., arg1=-3, arg2=-4) at /checkout/src/test/debuginfo/method-on-enum.rs:127
127         zzz(); // #break
$4 = {<No data fields>}
$5 = -3
$6 = -4

Breakpoint 1, method_on_enum::Enum::self_by_ref::h8e77efffda08da35 (self=0x5655a078, arg1=-5, arg2=-6) at /checkout/src/test/debuginfo/method-on-enum.rs:122
122         zzz(); // #break
$7 = {<No data fields>}
$8 = -5
$9 = -6

Breakpoint 2, method_on_enum::Enum::self_by_val::h153e020c0168d790 (self=..., arg1=-7, arg2=-8) at /checkout/src/test/debuginfo/method-on-enum.rs:127
127         zzz(); // #break
$10 = {<No data fields>}
$11 = -7
$12 = -8

Breakpoint 3, method_on_enum::Enum::self_owned::hb8ffc3e729c29c94 (self=0x5655a078, arg1=-9, arg2=-10) at /checkout/src/test/debuginfo/method-on-enum.rs:132
132         zzz(); // #break
$13 = {<No data fields>}
$14 = -9
$15 = -10
[Inferior 1 (process 243128) exited normally]
stderr: none


---- [debuginfo-gdb] debuginfo/simple-struct.rs stdout ----
---- [debuginfo-gdb] debuginfo/simple-struct.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = {x = 1000, y = -1001}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/simple-struct.gdb/simple-struct.debugger.script"
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
Breakpoint 1 at 0xb14: file /checkout/src/test/debuginfo/simple-struct.rs, line 229.
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/simple-struct.gdb/simple-struct.debugger.script:8: Error in sourced command file:
No symbol "simple_struct::NO_PADDING_16" in current context.


---- [debuginfo-gdb] debuginfo/simple-tuple.rs stdout ----
---- [debuginfo-gdb] debuginfo/simple-tuple.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = {__0 = -50, __1 = 50}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/simple-tuple.gdb/simple-tuple.debugger.script"
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
Breakpoint 1 at 0xa3a: file /checkout/src/test/debuginfo/simple-tuple.rs, line 207.
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/simple-tuple.gdb/simple-tuple.debugger.script:8: Error in sourced command file:
No symbol "simple_tuple::NO_PADDING_8" in current context.


---- [debuginfo-gdb] debuginfo/union-smoke.rs stdout ----
---- [debuginfo-gdb] debuginfo/union-smoke.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $2 = {a = {__0 = 1 '\001', __1 = 1 '\001'}, b = 257}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/union-smoke.gdb/union-smoke.debugger.script"
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
Breakpoint 1 at 0x827: file /checkout/src/test/debuginfo/union-smoke.rs, line 44.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, union_smoke::main::h2d7eac59282ab79f () at /checkout/src/test/debuginfo/union-smoke.rs:44
44     zzz(); // #break
$1 = {a = {__0 = 2 '\002', __1 = 2 '\002'}, b = 514}
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/union-smoke.gdb/union-smoke.debugger.script:10: Error in sourced command file:
No symbol "SU" in namespace "union_smoke".


---- [debuginfo-gdb] debuginfo/vec.rs stdout ----
---- [debuginfo-gdb] debuginfo/vec.rs stdout ----
