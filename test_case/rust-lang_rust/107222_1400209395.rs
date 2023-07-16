
make(3039)───cargo(3046)─┬─rustc(22176)───{rustc}(22177)
                         ├─{cargo}(3076)
                         └─{cargo}(22175)

3422881371: gdb -p 22176 {
Thread 2 (Thread 0x7eff51486700 (LWP 22177)):
#0  __lll_lock_wait_private () at ../nptl/sysdeps/unix/sysv/linux/x86_64/lowlevellock.S:95
#1  0x00007eff5753141f in _L_lock_9249 () from /lib/libc.so.6
#2  0x00007eff5752eea2 in __GI___libc_malloc (bytes=139634956828704) at malloc.c:2884
#3  0x00007eff5bff2339 in _dl_map_object_deps (map=0x7eff5c1e34c8, preloads=<optimized out>, npreloads=<optimized out>, trace_mode=0, open_mode=-2147483648)
    at dl-deps.c:511
#4  0x00007eff5bff84b1 in dl_open_worker (a=0x7eff4c234c90) at dl-open.c:261
#5  0x00007eff5bff4362 in _dl_catch_error (objname=0x7eff4c234cd8, errstring=0x7eff4c234ce0, mallocedp=0x7eff4c234cef, 
    operate=0x7eff5bff83a0 <dl_open_worker>, args=0x7eff4c234c90) at dl-error.c:187
#6  0x00007eff5bff7e6a in _dl_open (file=0x7eff575dee22 "libgcc_s.so.1", mode=-2147483647, caller_dlopen=0x7eff5759db75 <init+21>, nsid=-2, argc=244, 
    argv=<optimized out>, env=0x7fff3a6300b0) at dl-open.c:650
#7  0x00007eff575c6fb2 in do_dlopen (ptr=0x7eff4c234ed0) at dl-libc.c:87
#8  0x00007eff5bff4362 in _dl_catch_error (objname=0x7eff4c234ea8, errstring=0x7eff4c234eb0, mallocedp=0x7eff4c234ebf, operate=0x7eff575c6f70 <do_dlopen>, 
    args=0x7eff4c234ed0) at dl-error.c:187
#9  0x00007eff575c704f in dlerror_run (operate=<optimized out>, args=<optimized out>) at dl-libc.c:46
#10 0x00007eff575c70c1 in __GI___libc_dlopen_mode (name=<optimized out>, mode=<optimized out>) at dl-libc.c:163
#11 0x00007eff5759db75 in init () at ../sysdeps/x86_64/backtrace.c:52
#12 0x00007eff526a7be3 in pthread_once () at ../nptl/sysdeps/unix/sysv/linux/x86_64/pthread_once.S:103
#13 0x00007eff5759dc94 in __GI___backtrace (array=<optimized out>, size=256) at ../sysdeps/x86_64/backtrace.c:103
#14 0x00007eff582bb913 in _RNvNtCsjrrZhBKzCED_12rustc_driver14signal_handler17print_stack_trace.llvm.15726370174666866997 ()
   from /usr/lib/librustc_driver-71e17984e74646f8.so
#15 <signal handler called>
#16 0x00007eff5752babd in malloc_consolidate (av=0x7eff4c000020) at malloc.c:4162
#17 malloc_consolidate (av=0x7eff4c000020) at malloc.c:4100
#18 0x00007eff5752c538 in _int_free (av=0x7eff4c000020, p=0x7eff4c1790e0, have_lock=0) at malloc.c:4054
#19 0x00007eff585398d6 in _RNvXsi_NtCsebA1Mtihj7J_5alloc2rcINtB5_2RcNtNtCsfydfqF69Ojs_13rustc_session6cstore11CrateSourceENtNtNtCseracCQdR9rx_4core3ops4drop4Dr
op4dropCs9MOi0Jwv1hx_18rustc_codegen_llvm () from /usr/lib/librustc_driver-71e17984e74646f8.so
#20 0x00007eff58451fe2 in _RINvNtCseracCQdR9rx_4core3ptr13drop_in_placeNtCsg9Hoi51zSwQ_17rustc_codegen_ssa9CrateInfoECs9MOi0Jwv1hx_18rustc_codegen_llvm ()
   from /usr/lib/librustc_driver-71e17984e74646f8.so
#21 0x00007eff58464bbe in _RNvXs5_Cs9MOi0Jwv1hx_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsg9Hoi51zSwQ_17rustc_codegen_ssa6traits7backend14CodegenBac
kend4link () from /usr/lib/librustc_driver-71e17984e74646f8.so
#22 0x00007eff5835b4ef in _RNvMs1_NtCsDvyeRLlAnz_15rustc_interface7queriesNtB5_6Linker4link () from /usr/lib/librustc_driver-71e17984e74646f8.so
#23 0x00007eff58239307 in _RINvNtCsDvyeRLlAnz_15rustc_interface9interface23create_compiler_and_runINtNtCseracCQdR9rx_4core6result6ResultuNtCs6YPr06lYx8Y_12rust
c_errors15ErrorGuaranteedENCNvCsjrrZhBKzCED_12rustc_driver12run_compilers_0EB2A_ () from /usr/lib/librustc_driver-71e17984e74646f8.so
#24 0x00007eff582b54b1 in _RINvMs_CscOS01kbc7HE_10scoped_tlsINtB5_9ScopedKeyNtCsGvux78YpxL_10rustc_span14SessionGlobalsE3setNCINvNtCsDvyeRLlAnz_15rustc_interfa
ce9interface12run_compilerINtNtCseracCQdR9rx_4core6result6ResultuNtCs6YPr06lYx8Y_12rustc_errors15ErrorGuaranteedENCNvCsjrrZhBKzCED_12rustc_driver12run_compiler
s_0E0B2w_EB3Z_ () from /usr/lib/librustc_driver-71e17984e74646f8.so
#25 0x00007eff5826ea9f in _RINvNtNtCs8xX7TbgqUkD_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCsDvyeRLlAnz_15rustc_interface4util31run_in_th
read_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCseracCQdR9rx_4core6result6ResultuNtCs6YPr06lYx8Y_12rustc_errors15ErrorGuaranteedENCNvCsjrrZhBKzC
ED_12rustc_driver12run_compilers_0E0B31_E0B31_EB4u_ () from /usr/lib/librustc_driver-71e17984e74646f8.so
#26 0x00007eff5827384d in _RNSNvYNCINvMNtCs8xX7TbgqUkD_3std6threadNtBa_7Builder16spawn_unchecked_NCINvNtCsDvyeRLlAnz_15rustc_interface4util31run_in_thread_pool
_with_globalsNCINvNtB1d_9interface12run_compilerINtNtCseracCQdR9rx_4core6result6ResultuNtCs6YPr06lYx8Y_12rustc_errors15ErrorGuaranteedENCNvCsjrrZhBKzCED_12rust
c_driver12run_compilers_0E0B2S_E0B2S_Es_0INtNtNtB2X_3ops8function6FnOnceuE9call_once6vtableB4l_ () from /usr/lib/librustc_driver-71e17984e74646f8.so
#27 0x00007eff57898a53 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-3be94347da626c42.so
#28 0x00007eff526a21c7 in start_thread (arg=0x7eff51486700) at pthread_create.c:309
#29 0x00007eff57590b6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7eff5c07db80 (LWP 22176)):
#0  0x00007eff526a36bf in pthread_join (threadid=139635045459712, thread_return=0x0) at pthread_join.c:92
#1  0x00007eff57898cbd in std::sys::unix::thread::Thread::join () from /usr/lib/libstd-3be94347da626c42.so
#2  0x00007eff58273f84 in _RNvMs8_NtCs8xX7TbgqUkD_3std6threadINtB5_10JoinHandleINtNtCseracCQdR9rx_4core6result6ResultuNtCs6YPr06lYx8Y_12rustc_errors15ErrorGuar
anteedEE4joinCsjrrZhBKzCED_12rustc_driver () from /usr/lib/librustc_driver-71e17984e74646f8.so
#3  0x00007eff58238053 in _RINvNtCsDvyeRLlAnz_15rustc_interface9interface12run_compilerINtNtCseracCQdR9rx_4core6result6ResultuNtCs6YPr06lYx8Y_12rustc_errors15E
rrorGuaranteedENCNvCsjrrZhBKzCED_12rustc_driver12run_compilers_0EB2p_ () from /usr/lib/librustc_driver-71e17984e74646f8.so
#4  0x00007eff5822b786 in _RNvMs_CsjrrZhBKzCED_12rustc_driverNtB4_11RunCompiler3run () from /usr/lib/librustc_driver-71e17984e74646f8.so
#5  0x00007eff582ac507 in _RNvXsl_NtNtCseracCQdR9rx_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCNvCsjrrZhBKzCED_12rustc_driver4main0EINtNtNtB9_3ops8funct
ion6FnOnceuE9call_onceB1d_ () from /usr/lib/librustc_driver-71e17984e74646f8.so
#6  0x00007eff582333b7 in _RNvCsjrrZhBKzCED_12rustc_driver4main () from /usr/lib/librustc_driver-71e17984e74646f8.so
#7  0x0000558ff1e008dd in _RNvCs8j5PTPrFEub_10rustc_main4main ()
#8  0x0000558ff1e00913 in _RINvNtNtCs8xX7TbgqUkD_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECs8j5PTPrFEub_10rustc_main ()
#9  0x0000558ff1e00939 in _RNCINvNtCs8xX7TbgqUkD_3std2rt10lang_startuE0Cs8j5PTPrFEub_10rustc_main.llvm.8577511506139667028 ()
#10 0x00007eff578b8338 in std::rt::lang_start_internal () from /usr/lib/libstd-3be94347da626c42.so
#11 0x0000558ff1e00908 in main ()

} 3422881371

731432444: gdb -p 3046 {
Thread 3 (Thread 0x7f5de741e700 (LWP 22175)):
#0  0x00007f5de76f7ffd in poll () at ../sysdeps/unix/syscall-template.S:81
#1  0x000055a0eddae69e in _RNvNtNtCsfl6E6ueE1Ir_10cargo_util5read23imp5read2 ()
#2  0x000055a0eddab677 in _RNvMs_NtCsfl6E6ueE1Ir_10cargo_util15process_builderNtB4_14ProcessBuilder19exec_with_streaming ()
#3  0x000055a0ed93e07d in _RNvXs_NtNtCs18NhG74RG0h_5cargo4core8compilerNtB4_15DefaultExecutorNtB4_8Executor4exec ()
#4  0x000055a0ed6d8654 in _RNSNvYNCNvNtNtCs18NhG74RG0h_5cargo4core8compiler5rustcs0_0INtNtNtCseracCQdR9rx_4core3ops8function6FnOnceTRNtNtB8_9job_queue8JobState
EE9call_once6vtableBc_.llvm.4757161827961325619 ()
#5  0x000055a0ed6d6fac in _RNSNvYNCNvMNtNtNtCs18NhG74RG0h_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCseracCQdR9rx_4core3ops8function6FnOnceTRNtNtBb_9job_q
ueue8JobStateEE9call_once6vtableBf_.llvm.4757161827961325619 ()
#6  0x000055a0ed6d6fac in _RNSNvYNCNvMNtNtNtCs18NhG74RG0h_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCseracCQdR9rx_4core3ops8function6FnOnceTRNtNtBb_9job_q
ueue8JobStateEE9call_once6vtableBf_.llvm.4757161827961325619 ()
#7  0x000055a0ed725d1b in _RINvNtNtCs8xX7TbgqUkD_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvMs3_NtNtNtCs18NhG74RG0h_5cargo4core8compiler9job_
queueNtB1n_10DrainState3runs_0uEB1t_ ()
#8  0x000055a0ed8d3bf8 in _RNSNvYNCINvMNtCs8xX7TbgqUkD_3std6threadNtBa_7Builder16spawn_unchecked_NCNvMs3_NtNtNtCs18NhG74RG0h_5cargo4core8compiler9job_queueNtB1
e_10DrainState3runs_0uEs_0INtNtNtCseracCQdR9rx_4core3ops8function6FnOnceuE9call_once6vtableB1k_ ()
#9  0x000055a0eddd3f23 in std::sys::unix::thread::Thread::new::thread_start ()
#10 0x00007f5de7e291c7 in start_thread (arg=0x7f5de741e700) at pthread_create.c:309
#11 0x00007f5de7700b6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 2 (Thread 0x7f5de761f700 (LWP 3076)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055a0eddfded8 in std::sys::unix::futex::futex_wait ()
#2  0x000055a0edddcae7 in std::sys::unix::locks::futex_condvar::Condvar::wait ()
#3  0x000055a0eddc590e in _RINvMs3_CsfDQhADOhSzs_9jobserverNtB6_11HelperState16for_each_requestNCNCNvNtB6_3imp12spawn_helpers_00EB6_ ()
#4  0x000055a0eddc615c in _RINvNtNtCs8xX7TbgqUkD_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvNtCsfDQhADOhSzs_9jobserver3imp12spawn_helpers_0uE
B1l_.llvm.10851541821755940188 ()
#5  0x000055a0eddc6daf in _RNSNvYNCINvMNtCs8xX7TbgqUkD_3std6threadNtBa_7Builder16spawn_unchecked_NCNvNtCsfDQhADOhSzs_9jobserver3imp12spawn_helpers_0uEs_0INtNtN
tCseracCQdR9rx_4core3ops8function6FnOnceuE9call_once6vtableB1c_ ()
#6  0x000055a0eddd3f23 in std::sys::unix::thread::Thread::new::thread_start ()
#7  0x00007f5de7e291c7 in start_thread (arg=0x7f5de761f700) at pthread_create.c:309
#8  0x00007f5de7700b6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7f5de8ff0900 (LWP 3046)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055a0eddfded8 in std::sys::unix::futex::futex_wait ()
#2  0x000055a0edddcb71 in std::sys::unix::locks::futex_condvar::Condvar::wait_timeout ()
#3  0x000055a0ed91458e in _RINvMs_NtNtCs8xX7TbgqUkD_3std4sync7condvarNtB5_7Condvar18wait_timeout_whileINtNtNtCs18NhG74RG0h_5cargo4util5queue5StateNtNtNtNtB1i_4
core8compiler9job_queue7MessageENCNvMB1e_INtB1e_5QueueB1T_E3pop0EB1i_ ()
#4  0x000055a0ed65a13d in _RNvMNtNtCs18NhG74RG0h_5cargo4util5queueINtB2_5QueueNtNtNtNtB6_4core8compiler9job_queue7MessageE3popB6_ ()
#5  0x000055a0ed5db11c in _RNvMs3_NtNtNtCs18NhG74RG0h_5cargo4core8compiler9job_queueNtB5_10DrainState15drain_the_queue ()
#6  0x000055a0ed6deb09 in _RNvXsl_NtNtCseracCQdR9rx_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCINvNtNtCs8xX7TbgqUkD_3std6thread6scoped5scopeNCNvMs2_NtNt
NtCs18NhG74RG0h_5cargo4core8compiler9job_queueNtB22_8JobQueue7executes1_0INtNtB9_6result6ResultuNtCsbgIjWoq7kK3_6anyhow5ErrorEE0EINtNtNtB9_3ops8function6FnOnce
uE9call_onceB28_ ()
#7  0x000055a0ed926e33 in _RINvNtNtCs8xX7TbgqUkD_3std6thread6scoped5scopeNCNvMs2_NtNtNtCs18NhG74RG0h_5cargo4core8compiler9job_queueNtBQ_8JobQueue7executes1_0IN
tNtCseracCQdR9rx_4core6result6ResultuNtCsbgIjWoq7kK3_6anyhow5ErrorEEBW_ ()
#8  0x000055a0ed5d9c64 in _RNvMs2_NtNtNtCs18NhG74RG0h_5cargo4core8compiler9job_queueNtB5_8JobQueue7execute ()
#9  0x000055a0eda1b110 in _RNvMNtNtNtCs18NhG74RG0h_5cargo4core8compiler7contextNtB2_7Context7compile ()
#10 0x000055a0ed8ea3b6 in _RNvNtNtCs18NhG74RG0h_5cargo3ops13cargo_compile10compile_ws ()
#11 0x000055a0ed8ea0c6 in _RNvNtNtCs18NhG74RG0h_5cargo3ops13cargo_compile7compile ()
#12 0x000055a0ed5920b8 in _RNvNtNtCsdZtyED8dQzK_5cargo8commands5rustc4exec ()
#13 0x000055a0ed56285b in _RNvNtCsdZtyED8dQzK_5cargo3cli18execute_subcommand ()
#14 0x000055a0ed55fb73 in _RNvNtCsdZtyED8dQzK_5cargo3cli4main ()
#15 0x000055a0ed59b6df in _RNvCsdZtyED8dQzK_5cargo4main ()
#16 0x000055a0ed53d5a3 in _RINvNtNtCs8xX7TbgqUkD_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECsdZtyED8dQzK_5cargo ()
#17 0x000055a0ed53f489 in _RNCINvNtCs8xX7TbgqUkD_3std2rt10lang_startuE0CsdZtyED8dQzK_5cargo.llvm.3244771250985294454 ()
#18 0x000055a0edde5748 in std::rt::lang_start_internal ()
#19 0x000055a0ed59e238 in main ()
} 731432444

