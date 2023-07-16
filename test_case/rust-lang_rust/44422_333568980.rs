
    Finished release [optimized + debuginfo] target(s) in 273.72 secs
esafronov@s30h:~/code/cocaine-http-proxy$ RUST_LOG=cocaine=debug valgrind --track-origins=yes ./target/release/cocaine-http-proxy --config config.yaml
==1013426== Memcheck, a memory error detector
==1013426== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==1013426== Using Valgrind-3.10.1 and LibVEX; rerun with -h for copyright info
==1013426== Command: ./target/release/cocaine-http-proxy --config config.yaml
==1013426==
==1013426== Conditional jump or move depends on uninitialised value(s)
==1013426==    at 0x2C6E5E: choose_match_type (exec.rs:1128)
==1013426==    by 0x2C6E5E: regex::exec::ExecBuilder::build::h10ccf8b6b8299d88 (exec.rs:323)
==1013426==    by 0x2DCADD: build (re_builder.rs:75)
==1013426==    by 0x2DCADD: regex::re_unicode::Regex::new::h012ef141c6f18d27 (re_unicode.rs:185)
==1013426==    by 0x2AD9A7: new<cocaine::logging::Logger> (app.rs:57)
==1013426==    by 0x2AD9A7: cocaine_http_proxy::run::h3f5a24d7e7cc9e10 (lib.rs:214)
==1013426==    by 0x187D2A: cocaine_http_proxy::main::he8207dce7eaf6677 (main.rs:35)
==1013426==    by 0x3F584C: __rust_maybe_catch_panic (lib.rs:98)
==1013426==    by 0x3EEE9B: try<(),closure> (panicking.rs:458)
==1013426==    by 0x3EEE9B: catch_unwind<closure,()> (panic.rs:361)
==1013426==    by 0x3EEE9B: std::rt::lang_start::hc7d7ddefc6ff9283 (rt.rs:59)
==1013426==    by 0x5698EC4: (below main) (libc-start.c:287)
==1013426==  Uninitialised value was created by a stack allocation
==1013426==    at 0x2DCA10: regex::re_unicode::Regex::new::h012ef141c6f18d27 (re_unicode.rs:184)
==1013426==
==1013426== Thread 2 logging:
==1013426== Invalid read of size 8
==1013426==    at 0x31796B: drop_in_place<alloc::boxed::Box<Error>> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<std::io::error::Custom> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<alloc::boxed::Box<std::io::error::Custom>> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<std::io::error::Repr> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<std::io::error::Error> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<rmpv::decode::Error> (ptr.rs:60)
==1013426==    by 0x31796B: poll_recv<tokio_core::net::tcp::TcpStream> (lib.rs:660)
==1013426==    by 0x31796B: _$LT$cocaine..Multiplex$LT$T$GT$$u20$as$u20$futures..future..Future$GT$::poll::had0f1e7bc5b24b50 (lib.rs:724)
==1013426==    by 0x31F827: _$LT$cocaine..Supervisor$LT$R$GT$$u20$as$u20$futures..future..Future$GT$::poll::h578079ac3c98cdf6 (lib.rs:1245)
==1013426==    by 0x3221D0: _$LT$cocaine..Supervisor$LT$R$GT$$u20$as$u20$futures..future..Future$GT$::poll::h578079ac3c98cdf6 (lib.rs:1173)
==1013426==    by 0x301ADA: _$LT$futures..future..map_err..MapErr$LT$A$C$$u20$F$GT$$u20$as$u20$futures..future..Future$GT$::poll::h19a67496350f43a4 (map_err.rs:30)
==1013426==    by 0x3438E9: poll<Future> (mod.rs:113)
==1013426==    by 0x3438E9: {{closure}}<alloc::boxed::Box<Future>,alloc::arc::Arc<tokio_core::reactor::MySetReadiness>> (mod.rs:289)
==1013426==    by 0x3438E9: {{closure}}<alloc::boxed::Box<Future>,closure,core::result::Result<futures::poll::Async<()>, ()>> (mod.rs:350)
==1013426==    by 0x3438E9: set<closure,core::result::Result<futures::poll::Async<()>, ()>> (mod.rs:79)
==1013426==    by 0x3438E9: enter<alloc::boxed::Box<Future>,closure,core::result::Result<futures::poll::Async<()>, ()>> (mod.rs:350)
==1013426==    by 0x3438E9: poll_future_notify<alloc::boxed::Box<Future>,alloc::arc::Arc<tokio_core::reactor::MySetReadiness>> (mod.rs:289)
==1013426==    by 0x3438E9: {{closure}} (mod.rs:356)
==1013426==    by 0x3438E9: set<tokio_core::reactor::Core,closure,core::result::Result<futures::poll::Async<()>, ()>> (lib.rs:135)
==1013426==    by 0x3438E9: dispatch_task (mod.rs:355)
==1013426==    by 0x3438E9: dispatch (mod.rs:316)
==1013426==    by 0x3438E9: tokio_core::reactor::Core::poll::hf0c293cc4c2753a8 (mod.rs:304)
==1013426==    by 0x2E0840: run<futures::stream::fold::Fold<futures::stream::and_then::AndThen<futures::sync::mpsc::UnboundedReceiver<cocaine::logging::Event>, closure, alloc::boxed::Box<futures::future::result_::FutureResult<(), ()>>>, closure, futures::future::result_::FutureResult<i32, ()>, i32>> (mod.rs:241)
==1013426==    by 0x2E0840: {{closure}} (logging.rs:57)
==1013426==    by 0x2E0840: std::sys_common::backtrace::__rust_begin_short_backtrace::h78f62de96f5555b6 (backtrace.rs:136)
==1013426==    by 0x2E0D97: {{closure}}<closure,()> (mod.rs:364)
==1013426==    by 0x2E0D97: call_once<(),closure> (panic.rs:296)
==1013426==    by 0x2E0D97: std::panicking::try::do_call::h137de81e5c00f678 (panicking.rs:479)
==1013426==    by 0x3F584C: __rust_maybe_catch_panic (lib.rs:98)
==1013426==    by 0x2FBB43: try<(),std::panic::AssertUnwindSafe<closure>> (panicking.rs:458)
==1013426==    by 0x2FBB43: catch_unwind<std::panic::AssertUnwindSafe<closure>,()> (panic.rs:361)
==1013426==    by 0x2FBB43: {{closure}}<closure,()> (mod.rs:363)
==1013426==    by 0x2FBB43: _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hb81732eca627cead (boxed.rs:651)
==1013426==    by 0x3ED60B: call_once<(),()> (boxed.rs:661)
==1013426==    by 0x3ED60B: start_thread (thread.rs:21)
==1013426==    by 0x3ED60B: std::sys::imp::thread::Thread::new::thread_start::h0950d1160275a4d5 (thread.rs:84)
==1013426==    by 0x524B181: start_thread (pthread_create.c:312)
==1013426==    by 0x577147C: clone (clone.S:111)
==1013426==  Address 0x10f9814800000027 is not stack'd, malloc'd or (recently) free'd
==1013426==
==1013426==
==1013426== Process terminating with default action of signal 11 (SIGSEGV)
==1013426==  General Protection Fault
==1013426==    at 0x31796B: drop_in_place<alloc::boxed::Box<Error>> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<std::io::error::Custom> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<alloc::boxed::Box<std::io::error::Custom>> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<std::io::error::Repr> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<std::io::error::Error> (ptr.rs:60)
==1013426==    by 0x31796B: drop_in_place<rmpv::decode::Error> (ptr.rs:60)
==1013426==    by 0x31796B: poll_recv<tokio_core::net::tcp::TcpStream> (lib.rs:660)
==1013426==    by 0x31796B: _$LT$cocaine..Multiplex$LT$T$GT$$u20$as$u20$futures..future..Future$GT$::poll::had0f1e7bc5b24b50 (lib.rs:724)
==1013426==    by 0x31F827: _$LT$cocaine..Supervisor$LT$R$GT$$u20$as$u20$futures..future..Future$GT$::poll::h578079ac3c98cdf6 (lib.rs:1245)
==1013426==    by 0x3221D0: _$LT$cocaine..Supervisor$LT$R$GT$$u20$as$u20$futures..future..Future$GT$::poll::h578079ac3c98cdf6 (lib.rs:1173)
==1013426==    by 0x301ADA: _$LT$futures..future..map_err..MapErr$LT$A$C$$u20$F$GT$$u20$as$u20$futures..future..Future$GT$::poll::h19a67496350f43a4 (map_err.rs:30)
==1013426==    by 0x3438E9: poll<Future> (mod.rs:113)
==1013426==    by 0x3438E9: {{closure}}<alloc::boxed::Box<Future>,alloc::arc::Arc<tokio_core::reactor::MySetReadiness>> (mod.rs:289)
==1013426==    by 0x3438E9: {{closure}}<alloc::boxed::Box<Future>,closure,core::result::Result<futures::poll::Async<()>, ()>> (mod.rs:350)
==1013426==    by 0x3438E9: set<closure,core::result::Result<futures::poll::Async<()>, ()>> (mod.rs:79)
==1013426==    by 0x3438E9: enter<alloc::boxed::Box<Future>,closure,core::result::Result<futures::poll::Async<()>, ()>> (mod.rs:350)
==1013426==    by 0x3438E9: poll_future_notify<alloc::boxed::Box<Future>,alloc::arc::Arc<tokio_core::reactor::MySetReadiness>> (mod.rs:289)
==1013426==    by 0x3438E9: {{closure}} (mod.rs:356)
==1013426==    by 0x3438E9: set<tokio_core::reactor::Core,closure,core::result::Result<futures::poll::Async<()>, ()>> (lib.rs:135)
==1013426==    by 0x3438E9: dispatch_task (mod.rs:355)
==1013426==    by 0x3438E9: dispatch (mod.rs:316)
==1013426==    by 0x3438E9: tokio_core::reactor::Core::poll::hf0c293cc4c2753a8 (mod.rs:304)
==1013426==    by 0x2E0840: run<futures::stream::fold::Fold<futures::stream::and_then::AndThen<futures::sync::mpsc::UnboundedReceiver<cocaine::logging::Event>, closure, alloc::boxed::Box<futures::future::result_::FutureResult<(), ()>>>, closure, futures::future::result_::FutureResult<i32, ()>, i32>> (mod.rs:241)
==1013426==    by 0x2E0840: {{closure}} (logging.rs:57)
==1013426==    by 0x2E0840: std::sys_common::backtrace::__rust_begin_short_backtrace::h78f62de96f5555b6 (backtrace.rs:136)
==1013426==    by 0x2E0D97: {{closure}}<closure,()> (mod.rs:364)
==1013426==    by 0x2E0D97: call_once<(),closure> (panic.rs:296)
==1013426==    by 0x2E0D97: std::panicking::try::do_call::h137de81e5c00f678 (panicking.rs:479)
==1013426==    by 0x3F584C: __rust_maybe_catch_panic (lib.rs:98)
==1013426==    by 0x2FBB43: try<(),std::panic::AssertUnwindSafe<closure>> (panicking.rs:458)
==1013426==    by 0x2FBB43: catch_unwind<std::panic::AssertUnwindSafe<closure>,()> (panic.rs:361)
==1013426==    by 0x2FBB43: {{closure}}<closure,()> (mod.rs:363)
==1013426==    by 0x2FBB43: _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hb81732eca627cead (boxed.rs:651)
==1013426==    by 0x3ED60B: call_once<(),()> (boxed.rs:661)
==1013426==    by 0x3ED60B: start_thread (thread.rs:21)
==1013426==    by 0x3ED60B: std::sys::imp::thread::Thread::new::thread_start::h0950d1160275a4d5 (thread.rs:84)
==1013426==    by 0x524B181: start_thread (pthread_create.c:312)
==1013426==    by 0x577147C: clone (clone.S:111)
==1013426==
==1013426== HEAP SUMMARY:
==1013426==     in use at exit: 0 bytes in 0 blocks
==1013426==   total heap usage: 0 allocs, 0 frees, 0 bytes allocated
==1013426==
==1013426== All heap blocks were freed -- no leaks are possible
==1013426==
==1013426== For counts of detected and suppressed errors, rerun with: -v
==1013426== ERROR SUMMARY: 3 errors from 2 contexts (suppressed: 0 from 0)
Killed
