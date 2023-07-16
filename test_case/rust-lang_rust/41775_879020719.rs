
(gdb) mon machine Reset
Dwarf Error: Cannot find DIE at 0xccb referenced from DIE at 0xc31b [in module D:\Code\Xous\Core\loader\target\riscv32imac-unknown-none-elf\release\loader]
(gdb) c
Continuing.

Breakpoint 1, fillstack () at src/asm.S:24
24          la          t0, abort
(gdb) stepi
0x20501038      24          la          t0, abort
(gdb) stepi
25          csrw        mtvec, t0
(gdb) stepi
28          j   rust_entry
(gdb) stepi
Dwarf Error: Cannot find DIE at 0xccb referenced from DIE at 0xc31b [in module D:\Code\Xous\Core\loader\target\riscv32imac-unknown-none-elf\release\loader]
(gdb)
