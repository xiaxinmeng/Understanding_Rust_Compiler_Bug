plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:aa723573e04016ede7da6c5d7b029e72cb8a05a3)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
 finished in 13.588 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 145 tests
iiiiii.......i..i..ii.....i...i................i..F.........iiiii.F..ii.........i......i 88/145
failures:

Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [debuginfo-gdb] tests/debuginfo/generator-objects.rs stdout ----
---- [debuginfo-gdb] tests/debuginfo/generator-objects.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: $1 = generator_objects::main::{generator_env#0}::Unresumed{_ref__a: 0x[...]}
status: exit status: 0
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generator-objects.gdb/generator-objects.debugger.script"
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
Breakpoint 1 at 0x120b: file /checkout/tests/debuginfo/generator-objects.rs, line 86.
Breakpoint 2 at 0x1238: file /checkout/tests/debuginfo/generator-objects.rs, line 88.
Breakpoint 3 at 0x1265: file /checkout/tests/debuginfo/generator-objects.rs, line 90.
Breakpoint 4 at 0x1292: file /checkout/tests/debuginfo/generator-objects.rs, line 92.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, generator_objects::main () at /checkout/tests/debuginfo/generator-objects.rs:86
86     _zzz(); // #break
$1 = generator_objects::main::{generator_env#0}::Unresumed

Breakpoint 2, generator_objects::main () at /checkout/tests/debuginfo/generator-objects.rs:88
88     _zzz(); // #break
$2 = generator_objects::main::{generator_env#0}::Suspend0{c: 6, d: 7}

Breakpoint 3, generator_objects::main () at /checkout/tests/debuginfo/generator-objects.rs:90
90     _zzz(); // #break
$3 = generator_objects::main::{generator_env#0}::Suspend1{c: 7, d: 8}


Breakpoint 4, generator_objects::main () at /checkout/tests/debuginfo/generator-objects.rs:92
92     _zzz(); // #break
$4 = generator_objects::main::{generator_env#0}::Returned


 Inferior 1 [process 145495] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none


---- [debuginfo-gdb] tests/debuginfo/issue-57822.rs stdout ----
---- [debuginfo-gdb] tests/debuginfo/issue-57822.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: $2 = issue_57822::main::{generator_env#3}::Unresumed{a: issue_57822::main::{generator_env#2}::Unresumed{y: 2}}
status: exit status: 0
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/issue-57822.gdb/issue-57822.debugger.script"
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
Breakpoint 1 at 0x117f: file /checkout/tests/debuginfo/issue-57822.rs, line 50.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, issue_57822::main () at /checkout/tests/debuginfo/issue-57822.rs:50
50     zzz(); // #break
$1 = issue_57822::main::{closure_env#1} {f: issue_57822::main::{closure_env#0} {x: 1}}
$2 = issue_57822::main::{generator_env#3}::Unresumed


 Inferior 1 [process 145959] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none



