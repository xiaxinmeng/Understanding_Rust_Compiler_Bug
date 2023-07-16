
* thread #1: tid = 0xc417fe, 0x00000001000c3418 libstd-4e7c5e5c.dylib`je_extent_tree_ad_remove(rbtree=0x000000010039cce0, node=0x0000000000000000) + 216 at extent.c:38, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=1, address=0x18)
    frame #0: 0x00000001000c3418 libstd-4e7c5e5c.dylib`je_extent_tree_ad_remove(rbtree=0x000000010039cce0, node=0x0000000000000000) + 216 at extent.c:38
  * frame #1: 0x00000001000c4144 libstd-4e7c5e5c.dylib`je_huge_dalloc(ptr=<unavailable>) + 68 at huge.c:273
    frame #2: 0x00000001000ceb36 libstd-4e7c5e5c.dylib`je_sdallocx [inlined] je_isdalloct(ptr=<unavailable>) + 534 at jemalloc_internal.h:786
    frame #3: 0x00000001000ceb2b libstd-4e7c5e5c.dylib`je_sdallocx [inlined] je_isqalloc(ptr=<unavailable>) at jemalloc_internal.h:813
    frame #4: 0x00000001000ceb2b libstd-4e7c5e5c.dylib`je_sdallocx [inlined] isfree(ptr=<unavailable>) + 215 at jemalloc.c:1257
    frame #5: 0x00000001000cea54 libstd-4e7c5e5c.dylib`je_sdallocx(ptr=<unavailable>, size=<unavailable>, flags=<unavailable>) + 308 at jemalloc.c:1896
    frame #6: 0x00000001001053c9 libstd-4e7c5e5c.dylib`rust_try_inner + 9
    frame #7: 0x00000001001053b6 libstd-4e7c5e5c.dylib`rust_try + 6
    frame #8: 0x000000010009d43d libstd-4e7c5e5c.dylib`rt::lang_start::hd39f943feeb73f34NCz + 653
    frame #9: 0x00007fff93b1a5c9 libdyld.dylib`start + 1
