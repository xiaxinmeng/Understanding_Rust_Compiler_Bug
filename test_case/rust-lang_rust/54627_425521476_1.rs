
* thread #2, queue = 'com.apple.main-thread', stop reason = signal SIGSTOP
  * frame #0: 0x00007fff6dd86a16 libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff6df4f589 libsystem_pthread.dylib`_pthread_cond_wait + 732
    frame #2: 0x000000010074a922 cargo`std::thread::park::hcf011ba4ca76f30e + 258
    frame #3: 0x0000000100738f41 cargo`std::sync::mpsc::blocking::WaitToken::wait::hd9230f94c5255b6a + 49
    frame #4: 0x000000010016c201 cargo`_$LT$std..sync..mpsc..shared..Packet$LT$T$GT$$GT$::recv::h5f4c19e5378c2a13 + 721
    frame #5: 0x00000001000d8a59 cargo`_$LT$std..sync..mpsc..Receiver$LT$T$GT$$GT$::recv::hf216cfb8020f004f + 345
    frame #6: 0x00000001001a6092 cargo`cargo::core::compiler::job_queue::JobQueue::drain_the_queue::h3764126a2525930a + 6194
    frame #7: 0x000000010016bac3 cargo`crossbeam_utils::thread::scope::h9914efee12971b2d + 51
    frame #8: 0x000000010019f548 cargo`cargo::core::compiler::context::Context::compile::h0de288078e42e75f + 4680
    frame #9: 0x00000001001301c7 cargo`cargo::ops::cargo_compile::compile_ws::hf4ef6ca68174806f + 13351
    frame #10: 0x000000010012cd5a cargo`cargo::ops::cargo_compile::compile::hbce24daf0756c4b8 + 122
    frame #11: 0x000000010004802b cargo`cargo::commands::build::exec::h46dd9e3b7f9435f8 + 555
    frame #12: 0x000000010000d20f cargo`cargo::cli::main::h08f3a31a714d2f3b + 5759
    frame #13: 0x00000001000207db cargo`cargo::main::h325898d3a6f782ed + 235
    frame #14: 0x0000000100033086 cargo`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2d4a2fdaa5a923c6 + 6
    frame #15: 0x00000001007501a8 cargo`std::panicking::try::do_call::h32a103bebbb9cfdd (.llvm.3970480468671491771) + 24
    frame #16: 0x000000010075f49f cargo`__rust_maybe_catch_panic + 31
    frame #17: 0x0000000100745d9d cargo`std::rt::lang_start_internal::hb521492a178b86d1 + 237
    frame #18: 0x0000000100022abc cargo`main + 44
    frame #19: 0x0000000100001234 cargo`start + 52
  thread #3
    frame #0: 0x00007fff6dd8728a libsystem_kernel.dylib`__workq_kernreturn + 10
    frame #1: 0x00007fff6df4e009 libsystem_pthread.dylib`_pthread_wqthread + 1035
    frame #2: 0x00007fff6df4dbe9 libsystem_pthread.dylib`start_wqthread + 13
  thread #4
    frame #0: 0x00007fff6dd86a16 libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff6df4f589 libsystem_pthread.dylib`_pthread_cond_wait + 732
    frame #2: 0x000000010074a922 cargo`std::thread::park::hcf011ba4ca76f30e + 258
    frame #3: 0x0000000100738f41 cargo`std::sync::mpsc::blocking::WaitToken::wait::hd9230f94c5255b6a + 49
    frame #4: 0x0000000100415940 cargo`_$LT$std..sync..mpsc..stream..Packet$LT$T$GT$$GT$::recv::h877c907696a78808 + 528
    frame #5: 0x000000010041c221 cargo`_$LT$std..sync..mpsc..IntoIter$LT$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$::next::h10fa7f5bebcb5368 + 161
    frame #6: 0x000000010041743c cargo`std::sys_common::backtrace::__rust_begin_short_backtrace::h7e24be038b1900f2 + 140
    frame #7: 0x00000001004178a0 cargo`std::panicking::try::do_call::hf4e37f0cd706ffbf (.llvm.1982629740510497656) + 80
    frame #8: 0x000000010075f49f cargo`__rust_maybe_catch_panic + 31
    frame #9: 0x00000001004193d6 cargo`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h4086c5dfcb09db75 + 230
    frame #10: 0x000000010074f768 cargo`std::sys_common::thread::start_thread::ha87665a1386dceb1 + 136
    frame #11: 0x0000000100732c69 cargo`std::sys::unix::thread::Thread::new::thread_start::h23d0c1bead21a1d2 + 9
    frame #12: 0x00007fff6df4e661 libsystem_pthread.dylib`_pthread_body + 340
    frame #13: 0x00007fff6df4e50d libsystem_pthread.dylib`_pthread_start + 377
    frame #14: 0x00007fff6df4dbf9 libsystem_pthread.dylib`thread_start + 13
  thread #5
    frame #0: 0x00007fff6dd8809a libsystem_kernel.dylib`poll + 10
    frame #1: 0x00000001003d3f26 cargo`cargo::util::read2::imp::read2::ha8b8244bf595cdce + 326
    frame #2: 0x00000001001eff79 cargo`cargo::util::process_builder::ProcessBuilder::exec_with_streaming::h7deb28d3de80b685 + 585
    frame #3: 0x000000010033acfc cargo`_$LT$cargo..core..compiler..DefaultExecutor$u20$as$u20$cargo..core..compiler..Executor$GT$::exec_and_capture_output::he8f0e4879b39b8cc + 76
    frame #4: 0x000000010033824e cargo`_$LT$F$u20$as$u20$cargo..core..compiler..job..FnBox$LT$A$C$$u20$R$GT$$GT$::call_box::h734d6a29434fb575 + 2350
    frame #5: 0x00000001001a3eaa cargo`_$LT$F$u20$as$u20$cargo..core..compiler..job..FnBox$LT$A$C$$u20$R$GT$$GT$::call_box::h469ee540027384d0 + 58
    frame #6: 0x00000001001a3eaa cargo`_$LT$F$u20$as$u20$cargo..core..compiler..job..FnBox$LT$A$C$$u20$R$GT$$GT$::call_box::h469ee540027384d0 + 58
    frame #7: 0x00000001001a3f7f cargo`cargo::core::compiler::job::Job::run::h6e94002433c2c06c + 31
    frame #8: 0x00000001001749b2 cargo`_$LT$F$u20$as$u20$crossbeam_utils..thread..FnBox$LT$T$GT$$GT$::call_box::hb008eea6471bc5c6 + 178
    frame #9: 0x000000010075f49f cargo`__rust_maybe_catch_panic + 31
    frame #10: 0x0000000100093f4a cargo`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hfb41ae0dcf629060 + 154
    frame #11: 0x000000010074f768 cargo`std::sys_common::thread::start_thread::ha87665a1386dceb1 + 136
    frame #12: 0x0000000100732c69 cargo`std::sys::unix::thread::Thread::new::thread_start::h23d0c1bead21a1d2 + 9
    frame #13: 0x00007fff6df4e661 libsystem_pthread.dylib`_pthread_body + 340
    frame #14: 0x00007fff6df4e50d libsystem_pthread.dylib`_pthread_start + 377
    frame #15: 0x00007fff6df4dbf9 libsystem_pthread.dylib`thread_start + 13

