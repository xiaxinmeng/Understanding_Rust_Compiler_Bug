plain
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

failures:

---- [debuginfo-gdb] src/test/debuginfo/c-style-enum.rs stdout ----
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


---- [debuginfo-gdb] src/test/debuginfo/basic-types-mut-globals.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/basic-types-mut-globals.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = false
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-mut-globals.gdb/basic-types-mut-globals.debugger.script"
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
Breakpoint 1 at 0x854: file /checkout/src/test/debuginfo/basic-types-mut-globals.rs, line 116.
Breakpoint 2 at 0x90c: file /checkout/src/test/debuginfo/basic-types-mut-globals.rs, line 135.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_mut_globals::main::hdc1e01871e53fda2 () at /checkout/src/test/debuginfo/basic-types-mut-globals.rs:116
116     _zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-mut-globals.gdb/basic-types-mut-globals.debugger.script:10: Error in sourced command file:
No symbol "basic_types_mut_globals::B" in current context.


---- [debuginfo-gdb] src/test/debuginfo/basic-types-globals.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/basic-types-globals.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = false
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals.gdb/basic-types-globals.debugger.script"
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
Breakpoint 1 at 0x737: file /checkout/src/test/debuginfo/basic-types-globals.rs, line 70.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_globals::main::hbc8d68058c8fd353 () at /checkout/src/test/debuginfo/basic-types-globals.rs:70
70     _zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals.gdb/basic-types-globals.debugger.script:9: Error in sourced command file:
No symbol "basic_types_globals::B" in current context.


---- [debuginfo-gdb] src/test/debuginfo/basic-types-globals-metadata.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/basic-types-globals-metadata.rs stdout ----
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
Breakpoint 1 at 0x747: file /checkout/src/test/debuginfo/basic-types-globals-metadata.rs, line 72.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_globals_metadata::main::h40cb61f9869a7dee () at /checkout/src/test/debuginfo/basic-types-globals-metadata.rs:72
72     _zzz(); // #break
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals-metadata.gdb/basic-types-globals-metadata.debugger.script:9: Error in sourced command file:
No symbol "basic_types_globals_metadata::B" in current context.


---- [debuginfo-gdb] src/test/debuginfo/by-value-non-immediate-argument.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/by-value-non-immediate-argument.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $5 = {__0 = 7, __1 = 8, __2 = 9.5, __3 = 10.5}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/by-value-non-immediate-argument.gdb/by-value-non-immediate-argument.debugger.script"
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
Breakpoint 1 at 0x8e0: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 84.
Breakpoint 2 at 0x91f: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 88.
Breakpoint 3 at 0x934: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 92.
Breakpoint 4 at 0x954: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 98.
Breakpoint 5 at 0x974: file /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs, line 110.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, by_value_non_immediate_argument::fun::h429d2589dc415153 (s=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:84
84     zzz(); // #break
$1 = {a = 1, b = 2.5}

Breakpoint 2, by_value_non_immediate_argument::fun_fun::h178ec6805b748927 () at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:88
88     zzz(); // #break
$2 = {a = 3, b = 4.5}
$4 = 6.5


Breakpoint 3, by_value_non_immediate_argument::tup::h9534294e0462a399 (a=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:92
92     zzz(); // #break
$5 = <optimized out>

Breakpoint 4, by_value_non_immediate_argument::new_type::h9264e873a294381f (a=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:98
98     zzz(); // #break
$6 = <optimized out>

Breakpoint 5, by_value_non_immediate_argument::by_val_enum::h475eedc801c62462 (x=...) at /checkout/src/test/debuginfo/by-value-non-immediate-argument.rs:110
110     zzz(); // #break
$7 = <optimized out>
[Inferior 1 (process 246252) exited normally]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/basic-types-metadata.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/basic-types-metadata.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: type = [...] ([...])
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-metadata.gdb/basic-types-metadata.debugger.script"
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
Breakpoint 1 at 0x15c2: file /checkout/src/test/debuginfo/basic-types-metadata.rs, line 79.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, basic_types_metadata::main::h718e825634af6b31 () at /checkout/src/test/debuginfo/basic-types-metadata.rs:79
79     _zzz(); // #break
type = ()
type = isize
type = i8
type = i16
type = i32
---
type = u32
type = u64
type = f32
type = f64
type = void (*)(void)
All functions matching regular expression "_yyy":
File /checkout/src/test/debuginfo/basic-types-metadata.rs:
File /checkout/src/test/debuginfo/basic-types-metadata.rs:
static void basic_types_metadata::_yyy::h566496457405edbf(void);
type = struct {closure_env#0} {
    <no data fields>
}
type = struct {closure_env#1} {
    bool *_ref__b;
}
type = struct {closure_env#2} {
    bool *_ref__b;
    isize *_ref__i;
}
[Inferior 1 (process 246303) exited with code 0145]
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/debuginfo/basic-types-metadata.rs:84:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------
------------------------------------------


---- [debuginfo-gdb] src/test/debuginfo/function-arg-initialization.rs stdout ----
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

 Inferior 1 [process 246635] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/lexical-scopes-in-block-expression.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/lexical-scopes-in-block-expression.rs stdout ----
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


---- [debuginfo-gdb] src/test/debuginfo/limited-debuginfo.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/limited-debuginfo.rs stdout ----
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

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/clone.rs:
static void core::clone::impls::_$LT$impl$u20$core..clone..Clone$u20$for$u20$i32$GT$::clone::h332a8963b6637229(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/cmp.rs:
static void core::cmp::impls::_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$i32$GT$::lt::h174baf41d12215d5(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/hint.rs:
static void core::hint::black_box::h8afdf46635c02b29(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/iter/range.rs:
static void _$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$::spec_next::h25421a4c18266b98(void);
static void _$LT$i32$u20$as$u20$core..iter..range..Step$GT$::forward_unchecked::h98389c2b48bd1104(void);
static void core::iter::range::_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$::next::h8e3165666d538b49(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/iter/traits/collect.rs:
static void _$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$::into_iter::he28fe109613a86df(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/mem/mod.rs:
static void core::mem::replace::h32a3ccf943436c1b(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/ops/function.rs:
static void core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h50295efe6a1dd373(void);
static void core::ops::function::FnOnce::call_once::h6912c067084765ab(void);
static void core::ops::function::FnOnce::call_once::hb765ec0d0da55d94(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/core/src/ptr/mod.rs:
static void core::ptr::drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$::h9d9ddb68ad819b3a(void);
static void core::ptr::read::h2222d0b5c5c7b3d4(void);
static void core::ptr::write::h23a80f8477f34254(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/std/src/process.rs:
static void _$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$::report::hf7b67108a4ec43ea(void);
static void _$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$::report::he4fb26ffdd1987a5(void);
static void std::process::ExitCode::to_i32::h820a7980912040f2(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/std/src/rt.rs:
static void std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h5accc0b8ddda43ec(void);
static void std::rt::lang_start::hd76d91bf96d2f1be(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/std/src/sys/unix/process/process_common.rs:
static void std::sys::unix::process::process_common::ExitCode::as_i32::h93c3c54e36df358b(void);

File /rustc/5891376e9c37035bac0606cca4af9c39aa213b30/library/std/src/sys_common/backtrace.rs:
static void std::sys_common::backtrace::__rust_begin_short_backtrace::h0b2e1dde389ab8a3(void);
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
[Inferior 1 (process 247255) exited normally]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/method-on-enum.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/method-on-enum.rs stdout ----
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
Breakpoint 1 at 0x14c8: file /checkout/src/test/debuginfo/method-on-enum.rs, line 122.
Breakpoint 2 at 0x1534: file /checkout/src/test/debuginfo/method-on-enum.rs, line 127.
Breakpoint 3 at 0x15a8: file /checkout/src/test/debuginfo/method-on-enum.rs, line 132.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, method_on_enum::Enum::self_by_ref::h8e77efffda08da35 (self=0xffffd1b8, arg1=-1, arg2=-2) at /checkout/src/test/debuginfo/method-on-enum.rs:122
122         zzz(); // #break
$1 = {<No data fields>}
$2 = -1
$3 = -2

Breakpoint 2, method_on_enum::Enum::self_by_val::h153e020c0168d790 (self=..., arg1=-3, arg2=-4) at /checkout/src/test/debuginfo/method-on-enum.rs:127
127         zzz(); // #break
$4 = <optimized out>
$5 = -3
$6 = -4

Breakpoint 1, method_on_enum::Enum::self_by_ref::h8e77efffda08da35 (self=0x5655a078, arg1=-5, arg2=-6) at /checkout/src/test/debuginfo/method-on-enum.rs:122
122         zzz(); // #break
$7 = {<No data fields>}
$8 = -5
$9 = -6

Breakpoint 2, method_on_enum::Enum::self_by_val::h153e020c0168d790 (self=..., arg1=-7, arg2=-8) at /checkout/src/test/debuginfo/method-on-enum.rs:127
127         zzz(); // #break
$10 = <optimized out>
$11 = -7
$12 = -8

Breakpoint 3, method_on_enum::Enum::self_owned::h7b7565130444b41e (self=0x5655a078, arg1=-9, arg2=-10) at /checkout/src/test/debuginfo/method-on-enum.rs:132
132         zzz(); // #break
$13 = {<No data fields>}
$14 = -9
$15 = -10
[Inferior 1 (process 247296) exited normally]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/option-like-enum.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/option-like-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = {RUST$ENCODED$ENUM$0$None = {__0 = 0x12345678}}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/option-like-enum.gdb/option-like-enum.debugger.script"
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
Breakpoint 1 at 0x9d7: file /checkout/src/test/debuginfo/option-like-enum.rs, line 160.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, option_like_enum::main::he48b9c68abe783ce () at /checkout/src/test/debuginfo/option-like-enum.rs:160
160     zzz(); // #break
$1 = {<No data fields>}
$2 = {<No data fields>}
$3 = {<No data fields>}
$4 = (isize *) 0x0
$5 = {<No data fields>}
$6 = (isize *) 0x0
$7 = {<No data fields>}
$8 = {<No data fields>}
[Inferior 1 (process 247565) exited normally]
stderr: none


---- [debuginfo-gdb] src/test/debuginfo/simple-struct.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/simple-struct.rs stdout ----
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


---- [debuginfo-gdb] src/test/debuginfo/simple-tuple.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/simple-tuple.rs stdout ----
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


---- [debuginfo-gdb] src/test/debuginfo/struct-in-enum.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/struct-in-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: $1 = {{RUST$ENUM$DISR = Case1, __0 = 0, __1 = {x = 2088533116, y = 2088533116, z = 31868}}, {RUST$ENUM$DISR = Case1, [...]}}
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/struct-in-enum.gdb/struct-in-enum.debugger.script"
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
Breakpoint 1 at 0xa33: file /checkout/src/test/debuginfo/struct-in-enum.rs, line 82.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, struct_in_enum::main::h3132032422c45c05 () at /checkout/src/test/debuginfo/struct-in-enum.rs:82
82     zzz(); // #break
$1 = {<No data fields>}
$2 = {<No data fields>}
$3 = {<No data fields>}
A debugging session is active.

 Inferior 1 [process 247950] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none

