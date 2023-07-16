
/usr/powerpc64le-linux-gnu/lib/libc.a(fileops.o): In function `_IO_new_file_underflow':
(.text+0x628): undefined reference to `_Unwind_Resume'
/usr/powerpc64le-linux-gnu/lib/libc.a(printf_fp.o): In function `__printf_fp_l':
(.text+0x650): undefined reference to `__unordkf2'
(.text+0x68c): undefined reference to `__unordkf2'
(.text+0xcf8): undefined reference to `__lekf2'
/usr/powerpc64le-linux-gnu/lib/libc.a(printf_fphex.o): In function `__printf_fphex':
(.text+0xf8): undefined reference to `__unordkf2'
(.text+0x144): undefined reference to `__unordkf2'
(.text+0x164): undefined reference to `__lekf2'
/usr/powerpc64le-linux-gnu/lib/libc.a(iofputs.o): In function `_IO_fputs':
(.text+0x204): undefined reference to `_Unwind_Resume'
/usr/powerpc64le-linux-gnu/lib/libc.a(iofwrite.o): In function `_IO_fwrite':
(.text+0x2f4): undefined reference to `_Unwind_Resume'
/usr/powerpc64le-linux-gnu/lib/libc.a(iogetdelim.o): In function `_IO_getdelim':
(.text+0x460): undefined reference to `_Unwind_Resume'
Makefile:34: recipe for target 'hello_world.elf' failed
