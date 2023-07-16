 gdb
gdb --args rustc /root/.cargo/registry/src/github.com-1ecc6299db9ec823/gcc-0.3.35/src/lib.rs --crate-name gcc --crate-type lib -C opt-level=3 -C metadata=dcde155d15b2ddce -C extra-filename=-dcde155d15b2ddce --out-dir /root/xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server/target/release/deps --emit=dep-info,link -C linker=arm-linux-gnueabihf-gcc -L dependency=/root/xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server/target/release/deps --cap-lints allow
GNU gdb (Debian 7.11.1-2) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "arm-linux-gnueabihf".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from rustc...done.
(gdb) run
Starting program: /root/.cargo/bin/rustc /root/.cargo/registry/src/github.com-1ecc6299db9ec823/gcc-0.3.35/src/lib.rs --crate-name gcc --crate-type lib -C opt-level=3 -C metadata=dcde155d15b2ddce -C extra-filename=-dcde155d15b2ddce --out-dir /root/xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server/target/release/deps --emit=dep-info,link -C linker=arm-linux-gnueabihf-gcc -L dependency=/root/xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server/target/release/deps --cap-lints allow
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
[Inferior 1 (process 3472) exited with code 01]
