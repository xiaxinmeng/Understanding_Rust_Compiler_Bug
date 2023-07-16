
failures:

---- [debuginfo-gdb] debuginfo/c-style-enum.rs stdout ----
        NOTE: compiletest thinks it is using GDB version 7.7

error: line not found in debugger output: $1 = TheOnlyVariant

status: exit code: 0
command: gdb -quiet -batch -nx -command=x86_64-unknown-linux-gnu/test/debuginfo-gdb/c-style-enum.debugger.script
stdout:
------------------------------------------
GNU gdb (Ubuntu 7.7.1-0ubuntu5~14.04.2) 7.7.1
Copyright (C) 2014 Free Software Foundation, Inc.
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
Breakpoint 1 at 0x988: file src/test/debuginfo/c-style-enum.rs, line 156.

------------------------------------------
stderr:
------------------------------------------
x86_64-unknown-linux-gnu/test/debuginfo-gdb/c-style-enum.debugger.script:8: Error in sourced command file:
No symbol "c-style-enum::SINGLE_VARIANT" in current context.

------------------------------------------

thread '[debuginfo-gdb] debuginfo/c-style-enum.rs' panicked at 'explicit panic', /home/alex/code/rust3/src/compiletest/runtest.rs:1487



failures:
    [debuginfo-gdb] debuginfo/c-style-enum.rs

test result: FAILED. 91 passed; 1 failed; 4 ignored; 0 measured
