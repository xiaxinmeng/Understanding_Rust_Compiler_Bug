
==11962==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x559425363a3e in _$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$::fmt::h4b1f37077daa7a53 (/tmp/test+0xaa3e)
    #1 0x5594253da9c1 in core::fmt::Formatter::run::hb94f209be4566339 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libcore/fmt/mod.rs:1103:8
    #2 0x5594253da9c1 in core::fmt::write::h616689ac84e1a181 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libcore/fmt/mod.rs:1050
    #3 0x5594253c5dba in std::io::Write::write_fmt::hb8965c5d2ae4b514 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/io/mod.rs:1139:14
    #4 0x5594253c5dba in _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h6c417dd4a815f83a /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/io/stdio.rs:464
    #5 0x5594253c6d90 in std::io::stdio::print_to::_$u7b$$u7b$closure$u7d$$u7d$::hdaa16d922a855dbb /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/io/stdio.rs:694:8
    #6 0x5594253c6d90 in _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::hb14290590622978c /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/thread/local.rs:294
    #7 0x5594253c5f84 in std::io::stdio::print_to::ha0da5046fa9c3b6a /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/io/stdio.rs:688:17
    #8 0x5594253c5f84 in std::io::stdio::_print::h800c8e7b04169604 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/io/stdio.rs:709
    #9 0x559425363f10 in test::main::hcf302dac3ad452a9 (/tmp/test+0xaf10)
    #10 0x5594253649b7 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4a9b03456cc5fd09 (/tmp/test+0xb9b7)
    #11 0x5594253ced92 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h2fa83434b65add1b /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/rt.rs:59:12
    #12 0x5594253ced92 in _ZN3std9panicking3try7do_call17hd2b5f35b09cfe11fE.llvm.3794404441972262337 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/panicking.rs:310
    #13 0x5594253d76b9 in __rust_maybe_catch_panic /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libpanic_unwind/lib.rs:102:7
    #14 0x5594253beae5 in std::panicking::try::h7e30ff9eb6dacc39 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/panicking.rs:289:12
    #15 0x5594253beae5 in std::panic::catch_unwind::h088588b005d89c90 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/panic.rs:392
    #16 0x5594253beae5 in std::rt::lang_start_internal::hfc67e81213c1ae30 /rustc/8876906867b2db3c7177d69dd020c40d89177f86/src/libstd/rt.rs:58
    #17 0x559425364955 in std::rt::lang_start::h02d780a2abb945b7 (/tmp/test+0xb955)
    #18 0x559425363fd5 in main (/tmp/test+0xafd5)
    #19 0x7f9e04e0efaf in __libc_start_main /var/tmp/portage/sys-libs/glibc-2.26-r7/work/glibc-2.26/csu/../csu/libc-start.c:308
    #20 0x559425362449 in _start (/tmp/test+0x9449)

SUMMARY: MemorySanitizer: use-of-uninitialized-value (/tmp/test+0xaa3e) in _$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$::fmt::h4b1f37077daa7a53
Exiting
cielo tmp # rustc --version
rustc 1.30.0-nightly (887690686 2018-09-27)
