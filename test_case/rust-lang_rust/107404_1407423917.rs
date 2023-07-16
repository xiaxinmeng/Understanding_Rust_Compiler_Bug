plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
 finished in 15.923 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 145 tests
iiiiii.......i..i.ii......i.FFi...F............i............iiiii....ii.........i......i 88/145
failures:
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [debuginfo-gdb] tests/debuginfo/collapse-debuginfo-no-attr-flag.rs stdout ----
---- [debuginfo-gdb] tests/debuginfo/collapse-debuginfo-no-attr-flag.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: gdb failed to execute
status: exit status: 1
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/collapse-debuginfo-no-attr-flag.gdb/collapse-debuginfo-no-attr-flag.debugger.script"
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
Breakpoint 1 at 0x14a3: file /checkout/tests/debuginfo/collapse-debuginfo-no-attr-flag.rs, line 57.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
two


Breakpoint 1, collapse_debuginfo_no_attr_flag::main () at /checkout/tests/debuginfo/collapse-debuginfo-no-attr-flag.rs:57
57         three(); // #loc3
three
58         four(); // #loc4
#0  collapse_debuginfo_no_attr_flag::main () at /checkout/tests/debuginfo/collapse-debuginfo-no-attr-flag.rs:58
58         four(); // #loc4
60     std::process::exit(ret);
60     std::process::exit(ret);
#0  collapse_debuginfo_no_attr_flag::main () at /checkout/tests/debuginfo/collapse-debuginfo-no-attr-flag.rs:60
60     std::process::exit(ret);
[Inferior 1 (process 144784) exited normally]
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/collapse-debuginfo-no-attr-flag.gdb/collapse-debuginfo-no-attr-flag.debugger.script:16: Error in sourced command file:
No stack.


---- [debuginfo-gdb] tests/debuginfo/collapse-debuginfo-no-attr.rs stdout ----
---- [debuginfo-gdb] tests/debuginfo/collapse-debuginfo-no-attr.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: gdb failed to execute
status: exit status: 1
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/collapse-debuginfo-no-attr.gdb/collapse-debuginfo-no-attr.debugger.script"
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
Breakpoint 1 at 0x14c3: file /checkout/tests/debuginfo/collapse-debuginfo-no-attr.rs, line 56.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
two


Breakpoint 1, collapse_debuginfo_no_attr::main () at /checkout/tests/debuginfo/collapse-debuginfo-no-attr.rs:56
56         three(); // #loc3
three
57         four(); // #loc4
#0  collapse_debuginfo_no_attr::main () at /checkout/tests/debuginfo/collapse-debuginfo-no-attr.rs:57
57         four(); // #loc4
59     std::process::exit(ret);
59     std::process::exit(ret);
#0  collapse_debuginfo_no_attr::main () at /checkout/tests/debuginfo/collapse-debuginfo-no-attr.rs:59
59     std::process::exit(ret);
[Inferior 1 (process 144790) exited normally]
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/collapse-debuginfo-no-attr.gdb/collapse-debuginfo-no-attr.debugger.script:16: Error in sourced command file:
No stack.


---- [debuginfo-gdb] tests/debuginfo/collapse-debuginfo-with-attr-flag.rs stdout ----
---- [debuginfo-gdb] tests/debuginfo/collapse-debuginfo-with-attr-flag.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: gdb failed to execute
status: exit status: 1
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/collapse-debuginfo-with-attr-flag.gdb/collapse-debuginfo-with-attr-flag.debugger.script"
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
Breakpoint 1 at 0x14c3: file /checkout/tests/debuginfo/collapse-debuginfo-with-attr-flag.rs, line 59.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
two


Breakpoint 1, collapse_debuginfo_with_attr_flag::main () at /checkout/tests/debuginfo/collapse-debuginfo-with-attr-flag.rs:59
59         three(); // #loc3
three
60         four(); // #loc4
#0  collapse_debuginfo_with_attr_flag::main () at /checkout/tests/debuginfo/collapse-debuginfo-with-attr-flag.rs:60
60         four(); // #loc4
62     std::process::exit(ret);
62     std::process::exit(ret);
#0  collapse_debuginfo_with_attr_flag::main () at /checkout/tests/debuginfo/collapse-debuginfo-with-attr-flag.rs:62
62     std::process::exit(ret);
[Inferior 1 (process 144806) exited normally]
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/collapse-debuginfo-with-attr-flag.gdb/collapse-debuginfo-with-attr-flag.debugger.script:16: Error in sourced command file:
No stack.



failures:
