
cielo x264-rs (master)# rustc -Z sanitizer=memory test.rs
cielo x264-rs (master)# ./test
==3102==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x55ab16e331af in _$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$::fmt::h802b88056fe70792 (/usr/src/rust/x264-rs/test+0xb1af)
    #1 0x55ab16ec5e85 in core::fmt::write::h8ce42890c1b80a07 /checkout/src/libcore/fmt/mod.rs:931
    #2 0x55ab16eb41af in std::io::Write::write_fmt<std::io::stdio::StdoutLock> /checkout/src/libstd/io/mod.rs:1027
    #3 0x55ab16eb41af in _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::ha0edfaf608769622 /checkout/src/libstd/io/stdio.rs:460
    #4 0x55ab16eb486f in std::io::stdio::_print::h87ad81c40c1d9727 /checkout/src/libstd/io/stdio.rs:680
    #5 0x55ab16e33da8 in test::main::hb5b0b056a2b1b3fc (/usr/src/rust/x264-rs/test+0xbda8)
    #6 0x55ab16ebbda5 in core::ops::FnOnce::call_once<fn(),()> /checkout/src/libcore/ops.rs:2606
    #7 0x55ab16ebbda5 in std::panicking::try::do_call::h689a21caeeef92aa /checkout/src/libstd/panicking.rs:454
    #8 0x55ab16ec397a in __rust_maybe_catch_panic /checkout/src/libpanic_unwind/lib.rs:98
    #9 0x55ab16ebc58a in std::panicking::try<(),fn()> /checkout/src/libstd/panicking.rs:433
    #10 0x55ab16ebc58a in std::panic::catch_unwind<fn(),()> /checkout/src/libstd/panic.rs:361
    #11 0x55ab16ebc58a in std::rt::lang_start::hf63d494cb7dd034c /checkout/src/libstd/rt.rs:57
    #12 0x55ab16e33eaf in main (/usr/src/rust/x264-rs/test+0xbeaf)
    #13 0x7f8d6b60f720 in __libc_start_main (/lib64/libc.so.6+0x20720)
    #14 0x55ab16e31a58 in _start (/usr/src/rust/x264-rs/test+0x9a58)

SUMMARY: MemorySanitizer: use-of-uninitialized-value (/usr/src/rust/x264-rs/test+0xb1af) in _$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$::fmt::h802b88056fe70792
Exiting
