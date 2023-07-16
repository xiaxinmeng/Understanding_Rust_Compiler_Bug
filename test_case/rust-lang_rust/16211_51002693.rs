
$ cat lol.rs
extern crate rustc;
fn main() { fail!() }
$ rustc --version
rustc 0.12.0-pre (756b7b23c 2014-08-02 21:51:10 +0000)
$ rustc lol.rs -g
$ nm lol | ack rust_fail
$ gdb
GNU gdb (GDB) 7.7.1
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-unknown-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
(gdb) break rust_fail
No symbol table is loaded.  Use the "file" command.
Make breakpoint pending on future shared library load? (y or [n]) n
(gdb) q
$ 
