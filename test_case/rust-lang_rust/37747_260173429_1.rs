
* thread #1: tid = 0x54f6ef, 0x0000000100000fb4 lcfail, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=2, address=0x100000fb0)
    frame #0: 0x0000000100000fb4 lcfail
->  0x100000fb4: addl   %eax, (%rax)
    0x100000fb6: addb   %al, (%rax)
    0x100000fb8: sbbb   $0x0, %al
    0x100000fba: addb   %al, (%rax)
(lldb) bt
* thread #1: tid = 0x54f6ef, 0x0000000100000fb4 lcfail, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=2, address=0x100000fb0)
  * frame #0: 0x0000000100000fb4 lcfail
    frame #1: 0x00007fff90d4b5ad libdyld.dylib`start + 1
    frame #2: 0x00007fff90d4b5ad libdyld.dylib`start + 1
