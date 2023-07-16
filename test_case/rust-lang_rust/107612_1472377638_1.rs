gdb
PS C:\msys64\home\William\Projects\MCVE\msp430-shl> cargo run --release --target=msp430-none-elf -Zbuild-std=core
    Finished release [optimized + debuginfo] target(s) in 0.15s
     Running `msprun sim gdb target\msp430-none-elf\release\msp430-shl`
C:\msys64\opt\toolchains\bin\msp430-elf-gdb.exe: warning: Couldn't determine a path for the index cache directory.
Reading symbols from target\msp430-none-elf\release\msp430-shl...
Remote debugging using localhost:2000
0x0000ffff in __RESET_VECTOR ()
Erasing...
Loading section .text, size 0x20c lma 0xc000
Loading section .rodata, size 0x34 lma 0xc20c
Loading section .vector_table, size 0x20 lma 0xffe0
Start address 0x0000c000, load size 608
Transfer rate: 21 KB/sec, 202 bytes/write.
(gdb) c
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
rust_begin_unwind (_info=0x3ce) at src/lib.rs:21
21              msp430::asm::barrier();
(gdb) bt
#0  rust_begin_unwind (_info=0x3ce) at src/lib.rs:21
#1  0x0000c116 in core::panicking::panic_fmt (fmt=...) at src/panicking.rs:64
#2  0x0000c146 in core::panicking::panic (expr=...) at src/panicking.rs:114
#3  0x0000c0d6 in msp430_shl::shift_test () at src/main.rs:44
#4  msp430_shl::k707o76630f9enn0::k707o76630f9enn0 () at src/main.rs:28
#5  main () at src/main.rs:19
