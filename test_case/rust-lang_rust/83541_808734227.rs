
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
  * frame #0: 0x00007fff2a7cd631 libunwind.dylib`_Unwind_RaiseException
    frame #1: 0x0000000100009e48 luajit_test`lj_err_throw [inlined] err_raise_ext(errcode=<unavailable>) at lj_err.c:300:3 [opt]
    frame #2: 0x0000000100009e1c luajit_test`lj_err_throw(L=0x000000010022e380, errcode=2) at lj_err.c:514 [opt]
    frame #3: 0x0000000100009fbc luajit_test`lj_err_run(L=0x000000010022e380) at lj_err.c:623:3 [opt]
    frame #4: 0x000000010000a159 luajit_test`err_msgv(L=<unavailable>, em=<unavailable>) at lj_err.c:636:3 [opt]
    frame #5: 0x000000010000a1ff luajit_test`lj_err_optype(L=0x000000010022e380, o=<unavailable>, opm=LJ_ERR_OPINDEX) at lj_err.c:672:3 [opt]
    frame #6: 0x000000010000de8f luajit_test`lj_meta_tget(L=0x000000010022e380, o=0x000000010022fc38, k=0x000000010022fc40) at lj_meta.c:147:7 [opt]
    frame #7: 0x000000010001647f luajit_test`lua_gettable(L=0x000000010022e380, idx=<unavailable>) at lj_api.c:826:16 [opt]
    frame #8: 0x0000000100004094 luajit_test`luajit_test::ffi::lua_gettable2::h4d1817184f7e1edc(state=0x000000010022e380, idx=-2) at main.rs:16:9
    frame #9: 0x0000000100003f5c luajit_test`luajit_test::test_me::run_me::h21bdc8b42612ef14(state=0x000000010022e380) at main.rs:27:9
    frame #10: 0x0000000100005e05 luajit_test`lj_BC_FUNCC + 68
    frame #11: 0x0000000100016552 luajit_test`lua_pcall(L=<unavailable>, nargs=<unavailable>, nresults=<unavailable>, errfunc=<unavailable>) at lj_api.c:1169:12 [opt]
    frame #12: 0x0000000100003efc luajit_test`luajit_test::test_me::hea87b0187a61e91a at main.rs:32:13
    frame #13: 0x0000000100003f79 luajit_test`luajit_test::main::h82de456d29ced77f at main.rs:36:14
    frame #14: 0x000000010000410e luajit_test`core::ops::function::FnOnce::call_once::hbde686e067bb3c66((null)=(luajit_test`luajit_test::main::h82de456d29ced77f at main.rs:35), (null)=<unavailable>) at function.rs:227:5
    frame #15: 0x0000000100004021 luajit_test`std::sys_common::backtrace::__rust_begin_short_backtrace::h69ec8b0c6c660907(f=(luajit_test`luajit_test::main::h82de456d29ced77f at main.rs:35)) at backtrace.rs:125:18
    frame #16: 0x0000000100003e24 luajit_test`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hdd4e1056a49aed1e at rt.rs:66:18
    frame #17: 0x0000000100071d44 luajit_test`std::rt::lang_start_internal::h86f505dc7de50d93 [inlined] core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h0e377e204feaadc3 at function.rs:259:13 [opt]
    frame #18: 0x0000000100071d3d luajit_test`std::rt::lang_start_internal::h86f505dc7de50d93 [inlined] std::panicking::try::do_call::h9c5b8eda90bbe0b7 at panicking.rs:379 [opt]
    frame #19: 0x0000000100071d3d luajit_test`std::rt::lang_start_internal::h86f505dc7de50d93 [inlined] std::panicking::try::he150bdff9d5b30f2 at panicking.rs:343 [opt]
    frame #20: 0x0000000100071d3d luajit_test`std::rt::lang_start_internal::h86f505dc7de50d93 [inlined] std::panic::catch_unwind::h04e9c415c0907892 at panic.rs:431 [opt]
    frame #21: 0x0000000100071d3d luajit_test`std::rt::lang_start_internal::h86f505dc7de50d93 at rt.rs:51 [opt]
    frame #22: 0x0000000100003e01 luajit_test`std::rt::lang_start::h27e980249989fdcb(main=(luajit_test`luajit_test::main::h82de456d29ced77f at main.rs:35), argc=1, argv=0x00007ffeefbff788) at rt.rs:65:5
    frame #23: 0x0000000100003fa2 luajit_test`main + 34
    frame #24: 0x00007fff2061b621 libdyld.dylib`start + 1
    frame #25: 0x00007fff2061b621 libdyld.dylib`start + 1
