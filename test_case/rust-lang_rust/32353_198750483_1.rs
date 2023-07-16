
#10 0x000000006dfbea46 in core::panicking::panic_fmt (fmt=...,
    file_line=0x4a8fd4 <__imp__ZN6result13unwrap_failed10_FILE_LINE20h3f821f2b172d903fZVOE>) at src/libcore/panicking.rs:69
#11 0x0000000000426080 in compiletest::result::unwrap_failed<collections::string::FromUtf8Error> (msg=..., error=...) at src/libcore/macros.rs:29
#12 0x0000000000425ec1 in compiletest::result::Result<T, E>::unwrap (self=...)
    at src/libcore/result.rs:687
#13 0x0000000000422e20 in compiletest::procsrv::run (lib_path=..., prog=...,
    aux_path=..., args=..., env=..., input=...)
    at C:/msys64/home/we/rust/src/compiletest/procsrv.rs:68
#14 0x000000000046a92a in compiletest::runtest::program_output (
    config=0x419eeb8, testpaths=0x419f6a8, lib_path=..., prog=...,
    aux_path=..., args=..., env=..., input=...)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:1560
#15 0x00000000004500fa in compiletest::runtest::compose_and_run (
    config=0x419eeb8, testpaths=0x419f6a8, args=..., prog=..., procenv=...,
    lib_path=..., aux_path=..., input=...)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:1423
#16 0x000000000044a046 in compiletest::runtest::exec_compiled_test (
    config=0x419eeb8, props=0x419ec28, testpaths=0x419f6a8)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:1307
#17 0x000000000044ad2a in compiletest::runtest::run_rpass_test_revision (
    config=0x419eeb8, props=0x419ec28, testpaths=0x419f6a8, revision=...)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:185
#18 0x000000000044ac34 in fn$LP$$RF$common..Config$C$$u20$$RF$header..TestProps$C$$u20$$RF$test..TestPaths$C$$u20$core..option..Option$LT$$RF$str$GT$$RP$$u20$$u7b$runtest..run_rpass_test_revision$u7d$::fn_pointer_shim.12692::hf51f107a9ef0100c () at C:/msys64/home/we/rust/src/compiletest/runtest.rs:93
#19 0x000000000044a709 in compiletest::runtest::for_each_revision<fn(&compiletest::common::Config, &compiletest::header::TestProps, &test::TestPaths, core::option::Option<&str>)> (config=0x419eeb8, props=0x419ec28, testpaths=0x419f6a8,
    op=0x44ac40 <compiletest::runtest::run_rpass_test_revision>)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:79
#20 0x000000000043ba7a in compiletest::runtest::run_rpass_test (
    config=0x419eeb8, props=0x419ec28, testpaths=0x419f6a8)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:172
#21 0x000000000041c7f4 in compiletest::runtest::run (config=...,
    testpaths=0x419f6a8)
    at C:/msys64/home/we/rust/src/compiletest/runtest.rs:54
#22 0x000000000041c091 in fnfn ()
    at C:/msys64/home/we/rust/src/compiletest/compiletest.rs:390
#23 0x000000000041ca1f in compiletest::boxed::F.FnBox<A>::call_box (
    self=0x6b19d0, args=0) at src/liballoc/boxed.rs:541
#24 0x000000006a24b1fe in test::boxed::Box<FnBox<A, Output = R>+ Send + 'a>.FnOnce<A>::call_once (self=..., args=0) at src/liballoc/boxed.rs:559
#25 0x000000006a24a46b in fnfn () at src/libtest/lib.rs:1102
#26 0x000000006a249e41 in fnfn () at src/libstd\thread/mod.rs:278
#27 0x000000006a249ddd in test::sys_common::unwind::try::try_fn<closure> (
    opt_closure=0x419fbb0 "") at src/libstd\sys/common\unwind/mod.rs:127
#28 0x000000006df40d88 in __rust_try ()
    at src/libstd\sys/common\unwind/seh64_gnu.rs:55
#29 0x000000006df40ac2 in fnfn (s=0x6534e0)
    at src/libstd\sys/common\unwind/mod.rs:148
#30 0x000000006df409bc in std::thread::local::LocalKey<T>::with<closure,core::result::Result<(), Box<Any>>> (
    self=0x6e24ba30 <panicking::PANIC_COUNT::hc3d4a2b5a612830akuA>, f=...)
    at src/libstd\thread/local.rs:211
#31 0x000000006df408d9 in std::sys_common::unwind::inner_try (
    f=0x6a249d90 <test::sys_common::unwind::try::try_fn<closure>>,
    data=0x419fbb0 "") at src/libstd\sys/common\unwind/mod.rs:133
#32 0x000000006a249d34 in test::sys_common::unwind::try<closure> (f=...)
    at src/libstd\sys/common\unwind/mod.rs:123
#33 0x000000006a249bab in fnfn () at src/libstd\thread/mod.rs:278
#34 0x000000006a24a63f in test::boxed::F.FnBox<A>::call_box (self=0x236da30,
    args=0) at src/liballoc/boxed.rs:541
#35 0x000000006df3150e in std::boxed::Box<FnBox<A, Output = R>+ 'a>.FnOnce<A>::call_once (self=..., args=0) at src/liballoc/boxed.rs:550
#36 0x000000006df3e2cd in std::sys_common::thread::start_thread (
    main=0x240b7b0) at src/libstd\sys/common/thread.rs:23
#37 0x000000006df72e03 in std::sys::thread::Thread::new::thread_start (
    main=0x240b7b0) at src/libstd\sys/windows/thread.rs:52
#38 0x000000006df72c73 in sys::thread::Thread::new::thread_start::h055d242044bc864aCGz () at src/libstd\sys/windows/thread.rs:55
#39 0x00007ffcd5ac13d2 in KERNEL32!BaseThreadInitThunk ()
   from C:\Windows\system32\kernel32.dll
#40 0x00007ffcd61c54e4 in ntdll!RtlUserThreadStart ()
   from C:\Windows\SYSTEM32\ntdll.dll
#41 0x0000000000000000 in ?? ()
