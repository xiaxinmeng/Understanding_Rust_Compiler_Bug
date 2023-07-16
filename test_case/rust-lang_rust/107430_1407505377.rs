
make(14574)───cargo(14579)─┬─rustc(324)───{rustc}(325)
                           ├─{cargo}(14636)
                           └─{cargo}(323)

975758500 gdb -p 14579 {

Thread 3 (Thread 0x7fc5735d4700 (LWP 323)):
#0  0x00007fc573aaeffd in poll () at ../sysdeps/unix/syscall-template.S:81
#1  0x000055d304dce2be in _RNvNtNtCsE4XdqlEWLo_10cargo_util5read23imp5read2 ()
#2  0x000055d304dcbf54 in _RNvMs_NtCsE4XdqlEWLo_10cargo_util15process_builderNtB4_14ProcessBuilder19exec_with_streaming ()
#3  0x000055d304968f3d in _RNvXs_NtNtCs2emSBifjvTe_5cargo4core8compilerNtB4_15DefaultExecutorNtB4_8Executor4exec ()
#4  0x000055d3047628e2 in _RNSNvYNCNvNtNtCs2emSBifjvTe_5cargo4core8compiler5rustcs0_0INtNtNtCsbb2Bk6zBd68_4core3ops8function6FnOnceTRNtNtB8_9job_queue8JobState
EE9call_once6vtableBc_.llvm.10530776814902389931 ()
#5  0x000055d304760ecc in _RNSNvYNCNvMNtNtNtCs2emSBifjvTe_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCsbb2Bk6zBd68_4core3ops8function6FnOnceTRNtNtBb_9job_q
ueue8JobStateEE9call_once6vtableBf_.llvm.10530776814902389931 ()
#6  0x000055d304760ecc in _RNSNvYNCNvMNtNtNtCs2emSBifjvTe_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCsbb2Bk6zBd68_4core3ops8function6FnOnceTRNtNtBb_9job_q
ueue8JobStateEE9call_once6vtableBf_.llvm.10530776814902389931 ()
#7  0x000055d3047d3c8b in _RINvNtNtCs5ue1oedm64P_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvMs4_NtNtNtCs2emSBifjvTe_5cargo4core8compiler9job_
queueNtB1n_10DrainState3runs_0uEB1t_ ()
#8  0x000055d3047de05f in _RNSNvYNCINvMNtCs5ue1oedm64P_3std6threadNtBa_7Builder16spawn_unchecked_NCNvMs4_NtNtNtCs2emSBifjvTe_5cargo4core8compiler9job_queueNtB1
e_10DrainState3runs_0uEs_0INtNtNtCsbb2Bk6zBd68_4core3ops8function6FnOnceuE9call_once6vtableB1k_ ()
#9  0x000055d304dee6c3 in std::sys::unix::thread::Thread::new::thread_start ()
#10 0x00007fc5741e01c7 in start_thread (arg=0x7fc5735d4700) at pthread_create.c:309
#11 0x00007fc573ab7b6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 2 (Thread 0x7fc5739d6700 (LWP 14636)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055d304df53c0 in std::sys::unix::locks::futex_condvar::Condvar::wait ()
#2  0x000055d304de59e7 in _RINvMs3_Csksw40TnEC34_9jobserverNtB6_11HelperState16for_each_requestNCNCNvNtB6_3imp12spawn_helpers_00EB6_ ()
#3  0x000055d304de635b in _RINvNtNtCs5ue1oedm64P_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvNtCsksw40TnEC34_9jobserver3imp12spawn_helpers_0uE
B1l_ ()
#4  0x000055d304de70cc in _RNSNvYNCINvMNtCs5ue1oedm64P_3std6threadNtBa_7Builder16spawn_unchecked_NCNvNtCsksw40TnEC34_9jobserver3imp12spawn_helpers_0uEs_0INtNtN
tCsbb2Bk6zBd68_4core3ops8function6FnOnceuE9call_once6vtableB1c_ ()
#5  0x000055d304dee6c3 in std::sys::unix::thread::Thread::new::thread_start ()
#6  0x00007fc5741e01c7 in start_thread (arg=0x7fc5739d6700) at pthread_create.c:309
#7  0x00007fc573ab7b6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7fc5753a7900 (LWP 14579)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055d304e1d925 in std::sys::unix::futex::futex_wait ()
#2  0x000055d304df545c in std::sys::unix::locks::futex_condvar::Condvar::wait_timeout ()
#3  0x000055d30494045a in _RINvMs_NtNtCs5ue1oedm64P_3std4sync7condvarNtB5_7Condvar18wait_timeout_whileINtNtNtCs2emSBifjvTe_5cargo4util5queue5StateNtNtNtNtB1i_4
core8compiler9job_queue7MessageENCNvMB1e_INtB1e_5QueueB1T_E3pop0EB1i_ ()
#4  0x000055d30470e76d in _RNvMNtNtCs2emSBifjvTe_5cargo4util5queueINtB2_5QueueNtNtNtNtB6_4core8compiler9job_queue7MessageE3popB6_ ()
#5  0x000055d3045e43fc in _RNvMs4_NtNtNtCs2emSBifjvTe_5cargo4core8compiler9job_queueNtB5_10DrainState15drain_the_queue ()
#6  0x000055d3047752df in _RNvXsl_NtNtCsbb2Bk6zBd68_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCINvNtNtCs5ue1oedm64P_3std6thread6scoped5scopeNCNvMs3_NtNt
NtCs2emSBifjvTe_5cargo4core8compiler9job_queueNtB22_8JobQueue7executes1_0INtNtB9_6result6ResultuNtCs5pVbNG3eFti_6anyhow5ErrorEE0EINtNtNtB9_3ops8function6FnOnce
uE9call_onceB28_ ()
#7  0x000055d304954a03 in _RINvNtNtCs5ue1oedm64P_3std6thread6scoped5scopeNCNvMs3_NtNtNtCs2emSBifjvTe_5cargo4core8compiler9job_queueNtBQ_8JobQueue7executes1_0IN
tNtCsbb2Bk6zBd68_4core6result6ResultuNtCs5pVbNG3eFti_6anyhow5ErrorEEBW_ ()
#8  0x000055d3045e167a in _RNvMs3_NtNtNtCs2emSBifjvTe_5cargo4core8compiler9job_queueNtB5_8JobQueue7execute ()
#9  0x000055d3045dbab2 in _RNvMNtNtNtCs2emSBifjvTe_5cargo4core8compiler7contextNtB2_7Context7compile ()
#10 0x000055d3045fdf50 in _RNvNtNtCs2emSBifjvTe_5cargo3ops13cargo_compile10compile_ws ()
#11 0x000055d3045fdc66 in _RNvNtNtCs2emSBifjvTe_5cargo3ops13cargo_compile7compile ()
#12 0x000055d30459af32 in _RNvNtNtCsb2ZXJABZyMT_5cargo8commands5rustc4exec ()
#13 0x000055d30456a169 in _RNvNtCsb2ZXJABZyMT_5cargo3cli4main ()
#14 0x000055d3045a678f in _RNvCsb2ZXJABZyMT_5cargo4main ()
#15 0x000055d304530613 in _RINvNtNtCs5ue1oedm64P_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECsb2ZXJABZyMT_5cargo ()
#16 0x000055d30453c309 in _RNCINvNtCs5ue1oedm64P_3std2rt10lang_startuE0Csb2ZXJABZyMT_5cargo.llvm.17413693263814735732 ()
#17 0x000055d304dfca24 in std::rt::lang_start_internal ()
#18 0x000055d3045a9355 in main ()

} 975758500

1375004404: gdb -p 324 {

Thread 2 (Thread 0x7f7f2b425700 (LWP 325)):
#0  __lll_lock_wait_private () at ../nptl/sysdeps/unix/sysv/linux/x86_64/lowlevellock.S:95
#1  0x00007f7f31be641f in _L_lock_9249 () from /lib/libc.so.6
#2  0x00007f7f31be3ea2 in __GI___libc_malloc (bytes=140184041553952) at malloc.c:2884
#3  0x00007f7f369b6339 in _dl_map_object_deps (map=0x7f7f36ba7990, preloads=<optimized out>, npreloads=<optimized out>, trace_mode=0, open_mode=-2147483648)
    at dl-deps.c:511
#4  0x00007f7f369bc4b1 in dl_open_worker (a=0x7f7f36bc3950) at dl-open.c:261
#5  0x00007f7f369b8362 in _dl_catch_error (objname=0x7f7f36bc3998, errstring=0x7f7f36bc39a0, mallocedp=0x7f7f36bc39af, 
    operate=0x7f7f369bc3a0 <dl_open_worker>, args=0x7f7f36bc3950) at dl-error.c:187
#6  0x00007f7f369bbe6a in _dl_open (file=0x7f7f31c93e22 "libgcc_s.so.1", mode=-2147483647, caller_dlopen=0x7f7f31c52b75 <init+21>, nsid=-2, argc=244, 
    argv=<optimized out>, env=0x7ffcad8dece0) at dl-open.c:650
#7  0x00007f7f31c7bfb2 in do_dlopen (ptr=0x7f7f36bc3b90) at dl-libc.c:87
#8  0x00007f7f369b8362 in _dl_catch_error (objname=0x7f7f36bc3b68, errstring=0x7f7f36bc3b70, mallocedp=0x7f7f36bc3b7f, operate=0x7f7f31c7bf70 <do_dlopen>, 
    args=0x7f7f36bc3b90) at dl-error.c:187
#9  0x00007f7f31c7c04f in dlerror_run (operate=<optimized out>, args=<optimized out>) at dl-libc.c:46
#10 0x00007f7f31c7c0c1 in __GI___libc_dlopen_mode (name=<optimized out>, mode=<optimized out>) at dl-libc.c:163
#11 0x00007f7f31c52b75 in init () at ../sysdeps/x86_64/backtrace.c:52
#12 0x00007f7f2c4ecbe3 in pthread_once () at ../nptl/sysdeps/unix/sysv/linux/x86_64/pthread_once.S:103
#13 0x00007f7f31c52c94 in __GI___backtrace (array=<optimized out>, size=256) at ../sysdeps/x86_64/backtrace.c:103
#14 0x00007f7f3299bd73 in _RNvNtCs7yniko8b3jr_12rustc_driver14signal_handler17print_stack_trace.llvm.17417949737449634579 ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#15 <signal handler called>
#16 0x00007f7f31be0abd in malloc_consolidate (av=0x7f7f24000020) at malloc.c:4162
#17 malloc_consolidate (av=0x7f7f24000020) at malloc.c:4100
#18 0x00007f7f31be1538 in _int_free (av=0x7f7f24000020, p=0x7f7f252816c0, have_lock=0) at malloc.c:4054
#19 0x00007f7f336bbab8 in _RNvXsi_NtCserCGWL3tgwo_5alloc2rcINtB5_2RcNtNtNtCslp6EnrTXj2a_14rustc_metadata5rmeta7decoder13CrateMetadataENtNtNtCsbb2Bk6zBd68_4core
3ops4drop4Drop4dropCsdGSehhlwCsS_13rustc_resolve () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#20 0x00007f7f3363190c in _RINvNtCsbb2Bk6zBd68_4core3ptr13drop_in_placeNtNtCslp6EnrTXj2a_14rustc_metadata7creader6CStoreECsdGSehhlwCsS_13rustc_resolve ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#21 0x00007f7f3298d5dc in _RINvNtCsbb2Bk6zBd68_4core3ptr13drop_in_placeNtNtCsk8rlMcU6O5O_12rustc_middle2ty18ResolverGlobalCtxtECs7yniko8b3jr_12rustc_driver ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#22 0x00007f7f3298cbac in _RINvNtCsbb2Bk6zBd68_4core3ptr13drop_in_placeNtNtCs1Zr7Co45u5W_15rustc_interface7queries7QueriesECs7yniko8b3jr_12rustc_driver ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#23 0x00007f7f32977eef in _RINvMs2_NtCs1Zr7Co45u5W_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCs7yniko8b3jr_12rustc_driver12run_compilers_0
s0_0INtNtCsbb2Bk6zBd68_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCsduSZgml60gU_12rustc_errors15ErrorGuaranteedEEB1n_ ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#24 0x00007f7f329092c3 in _RINvCs8BsmQdggCzD_10rustc_span15with_source_mapINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuarantee
dENCNCINvNtCs1Zr7Co45u5W_15rustc_interface9interface12run_compilerBJ_NCNvCs7yniko8b3jr_12rustc_driver12run_compilers_0E00EB3h_ ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#25 0x00007f7f3296ee51 in _RINvMs_CsiqFDsH97eRr_10scoped_tlsINtB5_9ScopedKeyNtCs8BsmQdggCzD_10rustc_span14SessionGlobalsE3setNCINvNtCs1Zr7Co45u5W_15rustc_inter
face9interface12run_compilerINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuaranteedENCNvCs7yniko8b3jr_12rustc_driver12run_compil
ers_0E0B2y_EB41_ () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#26 0x00007f7f32937d7f in _RINvNtNtCs5ue1oedm64P_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNCINvNtCs1Zr7Co45u5W_15rustc_interface4util31run_in
_thread_pool_with_globalsNCINvNtB1o_9interface12run_compilerINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuaranteedENCNvCs7yniko
8b3jr_12rustc_driver12run_compilers_0E0B34_E00B34_EB4x_ () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#27 0x00007f7f32918bae in _RNSNvYNCINvMNtCs5ue1oedm64P_3std6threadNtBa_7Builder16spawn_unchecked_NCNCINvNtCs1Zr7Co45u5W_15rustc_interface4util31run_in_thread_p
ool_with_globalsNCINvNtB1f_9interface12run_compilerINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuaranteedENCNvCs7yniko8b3jr_12r
ustc_driver12run_compilers_0E0B2V_E00B2V_Es_0INtNtNtB30_3ops8function6FnOnceuE9call_once6vtableB4o_ () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#28 0x00007f7f31f50f43 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-aff51e0127895800.so
#29 0x00007f7f2c4e71c7 in start_thread (arg=0x7f7f2b425700) at pthread_create.c:309
#30 0x00007f7f31c45b6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7f7f36b9fbc0 (LWP 324)):
#0  0x00007f7f2c4e86bf in pthread_join (threadid=140184163342080, thread_return=0x0) at pthread_join.c:92
#1  0x00007f7f31f511ad in std::sys::unix::thread::Thread::join () from /usr/lib/libstd-aff51e0127895800.so
#2  0x00007f7f32919674 in _RNvMs6_NtCs5ue1oedm64P_3std6threadINtB5_9JoinInnerINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuaran
teedEE4joinCs7yniko8b3jr_12rustc_driver () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#3  0x00007f7f32910825 in _RNvXsl_NtNtCsbb2Bk6zBd68_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCINvNtNtCs5ue1oedm64P_3std6thread6scoped5scopeNCINvNtCs1Zr
7Co45u5W_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB21_9interface12run_compilerINtNtB9_6result6ResultuNtCsduSZgml60gU_12rustc_errors15Error
GuaranteedENCNvCs7yniko8b3jr_12rustc_driver12run_compilers_0E0B3H_E0B3H_E0EINtNtNtB9_3ops8function6FnOnceuE9call_onceB4U_ ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#4  0x00007f7f3290eab3 in _RINvNtNtCs5ue1oedm64P_3std6thread6scoped5scopeNCINvNtCs1Zr7Co45u5W_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtBP_
9interface12run_compilerINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuaranteedENCNvCs7yniko8b3jr_12rustc_driver12run_compilers_
0E0B2u_E0B2u_EB3X_ () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#5  0x00007f7f3292f609 in _RINvNtCs1Zr7Co45u5W_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB4_9interface12run_compilerINtNtCsbb2Bk6zBd68_4cor
e6result6ResultuNtCsduSZgml60gU_12rustc_errors15ErrorGuaranteedENCNvCs7yniko8b3jr_12rustc_driver12run_compilers_0E0B1J_EB3c_ ()
   from /usr/lib/librustc_driver-25f9795c092d9bac.so
#6  0x00007f7f329784d5 in _RINvNtCs1Zr7Co45u5W_15rustc_interface9interface12run_compilerINtNtCsbb2Bk6zBd68_4core6result6ResultuNtCsduSZgml60gU_12rustc_errors15
ErrorGuaranteedENCNvCs7yniko8b3jr_12rustc_driver12run_compilers_0EB2q_ () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#7  0x00007f7f329a2956 in _RNvMs_Cs7yniko8b3jr_12rustc_driverNtB4_11RunCompiler3run () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#8  0x00007f7f32910a9b in _RNvXsl_NtNtCsbb2Bk6zBd68_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCNvCs7yniko8b3jr_12rustc_driver4main0EINtNtNtB9_3ops8funct
ion6FnOnceuE9call_onceB1d_ () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#9  0x00007f7f329a963c in _RNvCs7yniko8b3jr_12rustc_driver4main () from /usr/lib/librustc_driver-25f9795c092d9bac.so
#10 0x000055b5c2400887 in _RNvCsavT0kBYC4hR_10rustc_main4main ()
#11 0x000055b5c2400873 in _RINvNtNtCs5ue1oedm64P_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECsavT0kBYC4hR_10rustc_main ()
#12 0x000055b5c2400859 in _RNCINvNtCs5ue1oedm64P_3std2rt10lang_startuE0CsavT0kBYC4hR_10rustc_main.llvm.17136657388742014391 ()
#13 0x00007f7f31f6c0b4 in std::rt::lang_start_internal () from /usr/lib/libstd-aff51e0127895800.so
#14 0x000055b5c24008b8 in main ()

} 1375004404

