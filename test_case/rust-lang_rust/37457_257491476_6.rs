
0000000000363be0 R_X86_64_JUMP_SLOT  _ZN6goblin5elf643sym6impure41_$LT$impl$u20$goblin..elf64..sym..Sym$GT$11is_function17hc057ef1decd9a013E@@Base
0000000000363c50 R_X86_64_JUMP_SLOT  _ZN6goblin5elf323dyn6impure10get_needed17h5bfa48fbc82bee6bE@@Base
0000000000363c68 R_X86_64_JUMP_SLOT  _ZN65_$LT$goblin..mach..header..Header$u20$as$u20$core..fmt..Debug$GT$3fmt17h4f6bb3fbd2cfeb8fE@@Base
0000000000363ca0 R_X86_64_JUMP_SLOT  _ZN6goblin5elf326header6impure76_$LT$impl$u20$core..fmt..Debug$u20$for$u20$goblin..elf32..header..Header$GT$3fmt17hb163bd5dd067aacfE@@Base

rdr -l target/debug/libgoblin.so

ELF X86_64 DYN @ 0x5b9c0
Soname: 
Libraries (5)
    libdl.so.2
    libpthread.so.0
    libgcc_s.so.1
    libc.so.6
    ld-linux-x86-64.so.2

objdump -R /usr/lib/chromium/chromium | grep JUMP_SLOT
0000000008b0e7d8 R_X86_64_JUMP_SLOT  geteuid@GLIBC_2.2.5
0000000008b0e7e0 R_X86_64_JUMP_SLOT  rmdir@GLIBC_2.2.5
0000000008b0e7e8 R_X86_64_JUMP_SLOT  pwrite64@GLIBC_2.2.5
0000000008b0e7f0 R_X86_64_JUMP_SLOT  fchmod@GLIBC_2.2.5
0000000008b0e7f8 R_X86_64_JUMP_SLOT  ftruncate64@GLIBC_2.2.5

