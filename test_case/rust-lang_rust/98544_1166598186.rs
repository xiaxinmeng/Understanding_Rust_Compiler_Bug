plain
test [ui] src/test/ui/rfc-2497-if-let-chains/feature-gate.rs ... ok
test [ui] src/test/ui/rfc-2126-extern-absolute-paths/single-segment.rs ... ok
test [ui] src/test/ui/rfc-2294-if-let-guard/run-pass.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-88498.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/invalid-let-in-a-valid-let-context.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-92145.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-90722.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs#disallowed ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-93150.rs ... ok
---
test [ui] src/test/ui/rfc-2457/mod_file_nonascii_with_path_allowed.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/chains-without-let.rs ... ok
test [ui] src/test/ui/rfc-2093-infer-outlives/privacy.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/feature-gate.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/invalid-let-in-a-valid-let-context.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-88498.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-90722.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-93150.rs ... ok
test [ui] src/test/ui/rfc-2497-if-let-chains/issue-92145.rs ... ok
---
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

failures:

---- [debuginfo-gdb] src/test/debuginfo/numeric-types.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001

error: line not found in debugger output: [...]$1 = 11
status: exit status: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/numeric-types.gdb/numeric-types.debugger.script"
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
Breakpoint 1 at 0x1571: file /checkout/src/test/debuginfo/numeric-types.rs, line 286.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, numeric_types::main::h0759cd9c4a18020f () at /checkout/src/test/debuginfo/numeric-types.rs:286
286     zzz(); // #break
$1 = {__0 = 11 '\v'}
$2 = {__0 = 22}
$3 = {__0 = 33}
$4 = {__0 = 44}
$5 = {__0 = 0x00000000000000000000000000000037}
$6 = {__0 = 66}
$7 = {__0 = 77 'M'}
$8 = {__0 = 88}
$9 = {__0 = 99}
$10 = {__0 = 100}
$11 = {__0 = 111}
$12 = {__0 = 122}
A debugging session is active.

 Inferior 1 [process 249566] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none



