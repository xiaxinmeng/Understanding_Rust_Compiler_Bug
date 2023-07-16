rs
# In another terminal
$ gdb-multiarch target/mips-unknown-linux-gnu/debug/rust-102722
gdb> sysroot /usr/mips-linux-gnu
gdb> target remote localhost:23456
gdb> c
Program received signal SIGBUS, Bus error.
...
