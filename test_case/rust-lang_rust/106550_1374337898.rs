plain
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 145 tests
iiiiii.......i...iii......i...i................i............iiiii....ii.........i......i 88/145
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...............ii..i.i..i.......i..F.....i...............

---- [debuginfo-gdb] src/test/debuginfo/step-into-match.rs stdout ----
---- [debuginfo-gdb] src/test/debuginfo/step-into-match.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: [...]match_enum(Some(12));
status: exit status: 0
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/step-into-match.gdb/step-into-match.debugger.script"
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
Breakpoint 1 at 0x1274: file /checkout/src/test/debuginfo/step-into-match.rs, line 342.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:342
342     match_enum(Some(42)); // #break
step_into_match::match_enum (x=...) at /checkout/src/test/debuginfo/step-into-match.rs:359
359     match x {
360         Some(42) => 1,
359     match x {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:343
343     match_enum(Some(12));
step_into_match::match_enum (x=...) at /checkout/src/test/debuginfo/step-into-match.rs:359
359     match x {
361         Some(_) => 2,
359     match x {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:344
344     match_enum(None);
step_into_match::match_enum (x=...) at /checkout/src/test/debuginfo/step-into-match.rs:359
359     match x {
362         None => 3,
359     match x {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:346
346     match_int(1);
step_into_match::match_int (y=1) at /checkout/src/test/debuginfo/step-into-match.rs:367
367     match y {
370         1 => 3,
367     match y {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:347
347     match_int(2);
step_into_match::match_int (y=2) at /checkout/src/test/debuginfo/step-into-match.rs:367
367     match y {
371         _ => 4,
367     match y {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:348
348     match_int(0);
step_into_match::match_int (y=0) at /checkout/src/test/debuginfo/step-into-match.rs:367
367     match y {
369         0 => 2,
367     match y {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:349
349     match_int(-1);
step_into_match::match_int (y=-1) at /checkout/src/test/debuginfo/step-into-match.rs:367
367     match y {
368         -1 => 1,
367     match y {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:351
351     match_tuple(5, 12);
step_into_match::match_tuple (a=5, b=12) at /checkout/src/test/debuginfo/step-into-match.rs:376
376     match (a, b) {
379         (5, 12) => 3,
376     match (a, b) {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:352
352     match_tuple(29, 1);
step_into_match::match_tuple (a=29, b=1) at /checkout/src/test/debuginfo/step-into-match.rs:376
376     match (a, b) {
378         (29, _) => 2,
376     match (a, b) {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:353
353     match_tuple(12, 12);
step_into_match::match_tuple (a=12, b=12) at /checkout/src/test/debuginfo/step-into-match.rs:376
376     match (a, b) {
376     match (a, b) {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:354
354     match_tuple(42, 12);
354     match_tuple(42, 12);
step_into_match::match_tuple (a=42, b=12) at /checkout/src/test/debuginfo/step-into-match.rs:376
376     match (a, b) {
377         (42, 12) => 1,
376     match (a, b) {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:355
355     match_tuple(1, 9);
step_into_match::match_tuple (a=1, b=9) at /checkout/src/test/debuginfo/step-into-match.rs:376
376     match (a, b) {
380         (_, 9) => 4,
376     match (a, b) {
step_into_match::main () at /checkout/src/test/debuginfo/step-into-match.rs:356
A debugging session is active.


 Inferior 1 [process 145862] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none



