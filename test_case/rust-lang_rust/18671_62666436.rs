
Starting program: /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/keegan/string-cache/src/lib.rs --crate-name string_cache --crate-type lib -g -C metadata=086e86ac78e2db93 -C extra-filename=-086e86ac78e2db93 --out-dir /home/keegan/string-cache/target --dep-info /home/keegan/string-cache/target/.fingerprint/string_cache-086e86ac78e2db93/dep-lib-string_cache -L /home/keegan/string-cache/target -L /home/keegan/string-cache/target/deps --extern string_cache_macros=/home/keegan/string-cache/target/deps/libstring_cache_macros-bb3bab23b4b54c1f.so --extern xxhash=/home/keegan/string-cache/target/deps/libxxhash-0275196ad61a41f5.rlib --extern phf=/home/keegan/string-cache/target/deps/libphf-4fc01267f85abb4e.rlib --extern lazy_static=/home/keegan/string-cache/target/deps/liblazy_static-518089d616e0fe56.rlib --extern phf_mac=/home/keegan/string-cache/target/deps/libphf_mac-78b8694be35c3487.so -Z debug-llvm
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7fffeffff480 (LWP 7258)]

Program received signal SIGABRT, Aborted.
[Switching to Thread 0x7fffeffff480 (LWP 7258)]
0x00007ffff629e077 in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
56  ../nptl/sysdeps/unix/sysv/linux/raise.c: No such file or directory.
#0  0x00007ffff629e077 in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#1  0x00007ffff629f458 in __GI_abort () at abort.c:89
#2  0x00007ffff6297196 in __assert_fail_base (fmt=0x7ffff63cd8c8 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", assertion=assertion@entry=0x7ffff33214a8 "findOption(Name) == Values.size() && \"Option already exists!\"", 
    file=file@entry=0x7ffff33213f8 "/home/keegan/rust-master/src/llvm/include/llvm/Support/CommandLine.h", line=line@entry=682, 
    function=function@entry=0x7ffff361f040 <void llvm::cl::parser<llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)>::addLiteralOption<llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)>(char const*, llvm::ScheduleDAGSDNodes* (* const&)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level), char const*)::__PRETTY_FUNCTION__> "void llvm::cl::parser<DataType>::addLiteralOption(const char*, const DT&, const char*) [with DT = llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level); DataType = llvm::Sche"...) at assert.c:92
#3  0x00007ffff6297242 in __GI___assert_fail (assertion=0x7ffff33214a8 "findOption(Name) == Values.size() && \"Option already exists!\"", file=0x7ffff33213f8 "/home/keegan/rust-master/src/llvm/include/llvm/Support/CommandLine.h", 
    line=682, 
    function=0x7ffff361f040 <void llvm::cl::parser<llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)>::addLiteralOption<llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)>(char const*, llvm::ScheduleDAGSDNodes* (* const&)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level), char const*)::__PRETTY_FUNCTION__> "void llvm::cl::parser<DataType>::addLiteralOption(const char*, const DT&, const char*) [with DT = llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level); DataType = llvm::Sche"...) at assert.c:101
#4  0x00007ffff22d283e in void llvm::cl::parser<llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)>::addLiteralOption<llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)>(char const*, llvm::ScheduleDAGSDNodes* (* const&)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level), char const*) [clone .part.675] () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/./librustc_llvm-4e7c5e5c.so
#5  0x00007ffff294edf6 in llvm::RegisterPassParser<llvm::RegisterScheduler>::NotifyAdd(char const*, void* (*)(), char const*) () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/./librustc_llvm-4e7c5e5c.so
#6  0x00007fffe54bb364 in _GLOBAL__sub_I_SelectionDAGISel.cpp () from /home/keegan/string-cache/target/deps/libphf_mac-78b8694be35c3487.so
#7  0x00007ffff7dea9fa in call_init (l=<optimized out>, argc=argc@entry=31, argv=argv@entry=0x7fffffffdfc8, env=env@entry=0x7fffffffe0c8) at dl-init.c:78
#8  0x00007ffff7deaae3 in call_init (env=0x7fffffffe0c8, argv=0x7fffffffdfc8, argc=31, l=<optimized out>) at dl-init.c:36
#9  _dl_init (main_map=main_map@entry=0x7fffe8000c30, argc=31, argv=0x7fffffffdfc8, env=0x7fffffffe0c8) at dl-init.c:126
#10 0x00007ffff7deec48 in dl_open_worker (a=a@entry=0x7fffefff34a8) at dl-open.c:577
#11 0x00007ffff7dea8b4 in _dl_catch_error (objname=objname@entry=0x7fffefff3498, errstring=errstring@entry=0x7fffefff34a0, mallocedp=mallocedp@entry=0x7fffefff3497, operate=operate@entry=0x7ffff7dee970 <dl_open_worker>, 
    args=args@entry=0x7fffefff34a8) at dl-error.c:187
#12 0x00007ffff7dee43b in _dl_open (file=0x7fffe8000940 "/home/keegan/string-cache/target/deps/libphf_mac-78b8694be35c3487.so", mode=-2147483647, caller_dlopen=<optimized out>, nsid=-2, argc=31, argv=0x7fffffffdfc8, 
    env=0x7fffffffe0c8) at dl-open.c:661
#13 0x00007ffff1b3e02b in dlopen_doit (a=a@entry=0x7fffefff36c0) at dlopen.c:66
#14 0x00007ffff7dea8b4 in _dl_catch_error (objname=0x7fffe80008f0, errstring=0x7fffe80008f8, mallocedp=0x7fffe80008e8, operate=0x7ffff1b3dfd0 <dlopen_doit>, args=0x7fffefff36c0) at dl-error.c:187
#15 0x00007ffff1b3e5dd in _dlerror_run (operate=operate@entry=0x7ffff1b3dfd0 <dlopen_doit>, args=args@entry=0x7fffefff36c0) at dlerror.c:163
#16 0x00007ffff1b3e0c1 in __dlopen (file=<optimized out>, mode=<optimized out>) at dlopen.c:87
#17 0x00007ffff739040b in dynamic_lib::DynamicLibrary::open::closure.146863 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#18 0x00007ffff7390187 in dynamic_lib::dl::check_for_errors_in::h18143281231927692217 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#19 0x00007ffff738e380 in plugin::load::PluginLoader$LT$$x27a$GT$.Visitor$LT$$x27v$GT$::visit_view_item::h26a4e4ac682959d04HG () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#20 0x00007ffff733addb in plugin::load::load_plugins::ha46ba574bd208bb4UGG () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#21 0x00007ffff707ab95 in driver::driver::phase_2_configure_and_expand::he9255e536fe74ee58XB () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#22 0x00007ffff72fe0af in driver::driver::compile_input::h7387a66b34164d36eRB () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#23 0x00007ffff73828c7 in driver::run_compiler::h4366432774627d6evHF () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#24 0x00007ffff7380bfc in driver::run::closure.146233 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#25 0x00007ffff6a87ed8 in task::TaskBuilder$LT$S$GT$::try_future::closure.104795 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#26 0x00007ffff6a87de3 in task::TaskBuilder$LT$S$GT$::spawn_internal::closure.104766 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#27 0x00007ffff7bd7ad2 in task::NativeSpawner.Spawner::spawn::closure.2582 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/libnative-4e7c5e5c.so
#28 0x00007ffff66daa7c in rust_try_inner () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#29 0x00007ffff66daa66 in rust_try () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#30 0x00007ffff6679623 in unwind::try::hab4dfcd74914625emYc () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#31 0x00007ffff66794ec in task::Task::run::h1b9699558e05f256u4b () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#32 0x00007ffff7bd78d7 in task::NativeSpawner.Spawner::spawn::closure.2508 () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/libnative-4e7c5e5c.so
#33 0x00007ffff667ad05 in thread::thread_start::hb3ae908ccfc057b3rpc () from /home/keegan/rust-master/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#34 0x00007ffff16270a4 in start_thread (arg=0x7fffeffff480) at pthread_create.c:309
#35 0x00007ffff634ec2d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111
