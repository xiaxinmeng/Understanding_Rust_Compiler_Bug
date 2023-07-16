 bash
(gdb) run -VV
Starting program: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV
warning: Cannot call inferior functions, Linux kernel PaX protection forbids return to non-executable pages!
[New LWP 7481]
error: Option 'version' given more than once.
^C
Program received signal SIGINT, Interrupt.
0x0000033afe21e9ad in ?? ()
(gdb) bt full
#0  0x0000033afe21e9ad in ?? ()
No symbol table info available.
#1  0x000000000000000e in ?? ()
No symbol table info available.
#2  0x00000000fbc24120 in ?? ()
No symbol table info available.
#3  0x0000033afe21e8d0 in ?? ()
No symbol table info available.
#4  0x0000033afbb6bd28 in ?? ()
No symbol table info available.
#5  0x0000033afbc00000 in ?? ()
No symbol table info available.
#6  0x0000000000000000 in ?? ()
No symbol table info available.
