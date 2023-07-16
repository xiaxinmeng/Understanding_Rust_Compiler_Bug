
$ make check-stage1-debuginfo-gdb TESTNAME=evec-in-struct
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: using CC=gcc (CFG_CC)
cfg: no lualatex found, deferring to xelatex
cfg: no xelatex found, deferring to pdflatex
cfg: no pdflatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: no llnextgen found, omitting grammar-verification
cfg: including test rules
cfg: antlr4 not available, skipping lexer test...
run debuginfo-gdb [x86_64-unknown-linux-gnu]: x86_64-unknown-linux-gnu/stage1/bin/compiletest

running 1 test
test [debuginfo-gdb] debuginfo/evec-in-struct.rs ... FAILED

using metrics ratchet: tmp/check-stage1-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-debuginfo-gdb-metrics.json
result of ratchet: 0 metrics added, 0 removed, 0 improved, 0 regressed, 0 noise
updated ratchet file

failures:

---- [debuginfo-gdb] debuginfo/evec-in-struct.rs stdout ----
    NOTE: compiletest thinks it is using GDB version 7.7

    error: line not found in debugger output: $2 = {x = {6, 7, 8}, y = {{9, 10}, {11, 12}}}

    status: exit code: 0
    command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/evec-in-struct.debugger.script
    stdout:
    ------------------------------------------
    GNU gdb (GDB) Fedora 7.7.1-19.fc20
    Copyright (C) 2014 Free Software Foundation, Inc.
    License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
    This is free software: you are free to change and redistribute it.
    There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
    and "show warranty" for details.
    This GDB was configured as "x86_64-redhat-linux-gnu".
    Type "show configuration" for configuration details.
    For bug reporting instructions, please see:
    <http://www.gnu.org/software/gdb/bugs/>.
    Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.
    For help, type "help".
    Type "apropos word" to search for commands related to "word".
    Breakpoint 1 at 0xa80: file /home/bkoropoff/Source/rust/src/test/debuginfo/evec-in-struct.rs, line 112.
    static void evec-in-struct::zzz(void);
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib64/libthread_db.so.1".

    Breakpoint 1, evec-in-struct::zzz () at /home/bkoropoff/Source/rust/src/test/debuginfo/evec-in-struct.rs:112
    112 fn zzz() { () }
    evec-in-struct::main () at /home/bkoropoff/Source/rust/src/test/debuginfo/evec-in-struct.rs:110
    110 }
    $1 = {x = {0, 1, 2}, y = -3, z = {4.5, 5.5}}
    $2 = {x = {6, 7, 8}, y = {{9, 10}, {12, 0}}}
    $3 = {x = {13, 14}, y = {15, 16}}
    $4 = {x = {17, 18, 19, 20, 21}}
    $5 = {x = {22, 23}, y = {24, 25}}
    A debugging session is active.

        Inferior 1 [process 27683] will be killed.

    Quit anyway? (y or n) [answered Y; input not from terminal]

    ------------------------------------------
    stderr:
    ------------------------------------------

    ------------------------------------------

    task '[debuginfo-gdb] debuginfo/evec-in-struct.rs' failed at 'explicit failure', /home/bkoropoff/Source/rust/src/compiletest/runtest.rs:1431



failures:
    [debuginfo-gdb] debuginfo/evec-in-struct.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

task '<main>' failed at 'Some tests failed', /home/bkoropoff/Source/rust/src/compiletest/compiletest.rs:255
make: *** [tmp/check-stage1-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-debuginfo-gdb.ok] Error 101
