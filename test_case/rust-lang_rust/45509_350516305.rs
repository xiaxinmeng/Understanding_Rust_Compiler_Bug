
root@stadler:~# cat hello.rs 
fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}

root@stadler:~# rustc hello.rs
root@stadler:~# ./hello 
Bus error
root@stadler:~# gdb ./hello
GNU gdb (Debian 7.12-6+b1) 7.12.0.20161007-git
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "sparc64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./hello...(no debugging symbols found)...done.
(gdb) r
Starting program: /root/hello 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/sparc64-linux-gnu/libthread_db.so.1".

Program received signal SIGBUS, Bus error.
0x0000010000006064 in _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h4ca565533725b233 ()
(gdb) bt
#0  0x0000010000006064 in _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h4ca565533725b233 ()
#1  0x0000010000007390 in std::rt::lang_start::he07d85b827d35475 ()
#2  0x00000100000050e0 in main ()
(gdb)
