
(lldb) bt
* thread #1: tid = 0x796f84, 0x0000000100011abf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::ptr::swap_nonoverlapping_bytes::h90af7d9428701cbf + 16 at ptr.rs:237, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x0000000100011abf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::ptr::swap_nonoverlapping_bytes::h90af7d9428701cbf + 16 at ptr.rs:237 [opt]
    frame #1: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::ptr::swap_nonoverlapping::h644cc6a65b724c83 at ptr.rs:187 [opt]
    frame #2: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::mem::swap::h5a9cd575e67a6181 at mem.rs:634 [opt]
    frame #3: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] core::mem::replace::hb91d071e3288f217 at mem.rs:691 [opt]
    frame #4: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::init::hdc127b6594dc2f82 at local.rs:270 [opt]
    frame #5: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h35683f0829174f6d at local.rs:296 [opt]
    frame #6: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::hd95e5656daaba459 at local.rs:248 [opt]
    frame #7: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 [inlined] std::panicking::update_panic_count::h9566b85bd2bcf46e at panicking.rs:240 [opt]
    frame #8: 0x0000000100011aaf segfault`std::panicking::rust_panic_with_hook::hc2253a5bcda7eda7 + 143 at panicking.rs:437 [opt]
    frame #9: 0x00000001000116b1 segfault`std::panicking::begin_panic_fmt::h2a4ecdf0c76cf1c3 + 49 at panicking.rs:350 [opt]
    frame #10: 0x0000000100011673 segfault`rust_begin_unwind + 35 at panicking.rs:328 [opt]
    frame #11: 0x00000001000463d6 segfault`core::panicking::panic_fmt::hb253bce78d883c0b + 118 at panicking.rs:71 [opt]
    frame #12: 0x0000000100046354 segfault`core::panicking::panic_bounds_check::h80816a3e0195fd50 + 116 at panicking.rs:58 [opt]
    frame #13: 0x0000000100000af7 segfault`segfault::main::hdaaaca17ad5afac1 + 199 at main.rs:3
    frame #14: 0x0000000100000ba2 segfault`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd2353479c4883d87 + 18 at rt.rs:74
    frame #15: 0x0000000100011638 segfault`std::panicking::try::do_call::h780530fa434ed2ed [inlined] std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::hcfde3e1c19e6fa13 + 24 at rt.rs:59 [opt]
    frame #16: 0x000000010001162c segfault`std::panicking::try::do_call::h780530fa434ed2ed + 12 at panicking.rs:310 [opt]
    frame #17: 0x000000010001d2bf segfault`__rust_maybe_catch_panic + 31 at lib.rs:105 [opt]
    frame #18: 0x0000000100009c82 segfault`std::rt::lang_start_internal::h46f9f692ece7505b [inlined] std::panicking::try::h0c5a2273ff0725da + 51 at panicking.rs:289 [opt]
    frame #19: 0x0000000100009c4f segfault`std::rt::lang_start_internal::h46f9f692ece7505b [inlined] std::panic::catch_unwind::h3474c6e3bb49278a at panic.rs:374 [opt]
    frame #20: 0x0000000100009c4f segfault`std::rt::lang_start_internal::h46f9f692ece7505b + 191 at rt.rs:58 [opt]
    frame #21: 0x0000000100000b82 segfault`std::rt::lang_start::hd3629e9310e6f506(main=(segfault`segfault::main::hdaaaca17ad5afac1 at main.rs:1), argc=1, argv=0x00007fff5fbffa38) + 66 at rt.rs:74
    frame #22: 0x0000000100000b25 segfault`main + 37
    frame #23: 0x00007fff894935c9 libdyld.dylib`start + 1
    frame #24: 0x00007fff894935c9 libdyld.dylib`start + 1
