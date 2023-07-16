
$ cat foo.rs 
extern crate rustc;
fn main() { fail!() }
$ rustc -v
rustc 0.12.0-pre-nightly (25741603f 2014-08-03 23:46:10 +0000)
$ rustc foo.rs -g 
$ gdb ./foo 
GNU gdb (Ubuntu 7.7-0ubuntu3.1) 7.7
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
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./foo...done.
(gdb) b rust_fail
Function "rust_fail" not defined.
Make breakpoint pending on future shared library load? (y or [n]) y
Breakpoint 1 (rust_fail) pending.
(gdb) r
Starting program: /home/alex/code/rust2/foo 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
task '<main>' failed at 'explicit failure', foo.rs:2

Breakpoint 1, 0x00007ffff7849dc0 in rust_fail () from /home/alex/code/rust/lib/librustrt-4e7c5e5c.so
(gdb) 
