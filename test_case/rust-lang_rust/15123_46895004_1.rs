
$ lldb ./foo 
r
Current executable set to './foo' (x86_64).
(lldb) r
Process 48594 launched: './foo' (x86_64)
Process 48594 stopped
* thread #1: tid = 0xb9eef, 0x000000010006fa2c foo`je_mallocx [inlined] je_small_size2bin_lookup + 15 at arena.h:552, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x000000010006fa2c foo`je_mallocx [inlined] je_small_size2bin_lookup + 15 at arena.h:552
(lldb) bt
* thread #1: tid = 0xb9eef, 0x000000010006fa2c foo`je_mallocx [inlined] je_small_size2bin_lookup + 15 at arena.h:552, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x000000010006fa2c foo`je_mallocx [inlined] je_small_size2bin_lookup + 15 at arena.h:552
    frame #1: 0x000000010006fa1d foo`je_mallocx [inlined] je_small_size2bin + 13 at arena.h:564
    frame #2: 0x000000010006fa10 foo`je_mallocx [inlined] je_tcache_alloc_small at tcache.h:266
    frame #3: 0x000000010006fa10 foo`je_mallocx [inlined] je_arena_malloc + 215 at arena.h:1074
    frame #4: 0x000000010006f939 foo`je_mallocx [inlined] je_ipalloct(alignment=<unavailable>) + 22 at jemalloc_internal.h:686
    frame #5: 0x000000010006f923 foo`je_mallocx [inlined] imallocx(alignment=<unavailable>, arena=<unavailable>) at jemalloc.c:1373
    frame #6: 0x000000010006f923 foo`je_mallocx(size=<unavailable>, flags=<unavailable>) + 4051 at jemalloc.c:1456
    frame #7: 0x0000000100001765 foo`heap::imp::allocate::hc6636e5636f6474b8ba::v0.0 + 117
    frame #8: 0x00000001000016e2 foo`heap::allocate::hb186c7c7af3e0a9fQba::v0.0 + 66
    frame #9: 0x000000010000159e foo`main::hb85e3e3480a04aa0faa::v0.0 + 78
    frame #10: 0x0000000100046579 foo`start::closure.7382 + 105
    frame #11: 0x00000001000592f8 foo`task::Task::run::closure.5182 + 56
    frame #12: 0x000000010005c7dc foo`rust_try + 12
    frame #13: 0x000000010005af4a foo`unwind::try::hb6705ce8e0cb20d2LCd::v0.11.0.pre + 74
    frame #14: 0x00000001000591c5 foo`task::Task::run::hf9d6a095707c904bXSc::v0.11.0.pre + 101
    frame #15: 0x000000010004636e foo`start::h8ff60cbf268dcceaVre::v0.11.0.pre + 574
    frame #16: 0x000000010004611c foo`lang_start::h3504ba38985a4483fre::v0.11.0.pre + 124
    frame #17: 0x000000010000168f foo`main + 79
