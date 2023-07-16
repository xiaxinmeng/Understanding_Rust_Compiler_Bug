
make(8615)───cargo(8640)─┬─rustc(26720)───{rustc}(26721)
                         ├─{cargo}(8652)
                         └─{cargo}(26719)

9439759625: gdb -p 8640 {
Thread 3 (Thread 0x7fe3fb639700 (LWP 26719)):
#0  0x00007fe3fb912ffd in poll () at ../sysdeps/unix/syscall-template.S:81
#1  0x000055c2e41ac40e in _RNvNtNtCs6TdZoFU1KMl_10cargo_util5read23imp5read2 ()
#2  0x000055c2e41a9497 in _RNvMs_NtCs6TdZoFU1KMl_10cargo_util15process_builderNtB4_14ProcessBuilder19exec_with_streaming ()
#3  0x000055c2e3daaa7d in _RNvXs_NtNtCsMzOSVqp7KB_5cargo4core8compilerNtB4_15DefaultExecutorNtB4_8Executor4exec ()
#4  0x000055c2e3b51ddd in _RNSNvYNCNvNtNtCsMzOSVqp7KB_5cargo4core8compiler5rustcs0_0INtNtNtCs2vtRKDehYxH_4core3ops8function6FnOnceTRNtNtB8_9job_queue8JobStateE
E9call_once6vtableBc_.llvm.11378506646933048436 ()
#5  0x000055c2e3b5064c in _RNSNvYNCNvMNtNtNtCsMzOSVqp7KB_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCs2vtRKDehYxH_4core3ops8function6FnOnceTRNtNtBb_9job_qu
eue8JobStateEE9call_once6vtableBf_.llvm.11378506646933048436 ()
#6  0x000055c2e3b5064c in _RNSNvYNCNvMNtNtNtCsMzOSVqp7KB_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCs2vtRKDehYxH_4core3ops8function6FnOnceTRNtNtBb_9job_qu
eue8JobStateEE9call_once6vtableBf_.llvm.11378506646933048436 ()
#7  0x000055c2e3bc79fb in _RINvNtNtCsckbJKWyZekZ_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvMs3_NtNtNtCsMzOSVqp7KB_5cargo4core8compiler9job_q
ueueNtB1n_10DrainState3runs_0uEB1t_ ()
#8  0x000055c2e3bcd9b9 in _RNSNvYNCINvMNtCsckbJKWyZekZ_3std6threadNtBa_7Builder16spawn_unchecked_NCNvMs3_NtNtNtCsMzOSVqp7KB_5cargo4core8compiler9job_queueNtB1e
_10DrainState3runs_0uEs_0INtNtNtCs2vtRKDehYxH_4core3ops8function6FnOnceuE9call_once6vtableB1k_ ()
#9  0x000055c2e42071a3 in std::sys::unix::thread::Thread::new::thread_start ()
#10 0x00007fe3fc0441c7 in start_thread (arg=0x7fe3fb639700) at pthread_create.c:309
#11 0x00007fe3fb91bb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 2 (Thread 0x7fe3fb83a700 (LWP 8652)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055c2e41d9fa3 in std::sys::unix::locks::futex_condvar::Condvar::wait ()
#2  0x000055c2e41c385e in _RINvMs3_CsbMW2GzUNIV5_9jobserverNtB6_11HelperState16for_each_requestNCNCNvNtB6_3imp12spawn_helpers_00EB6_ ()
#3  0x000055c2e41c3fdc in _RINvNtNtCsckbJKWyZekZ_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvNtCsbMW2GzUNIV5_9jobserver3imp12spawn_helpers_0uE
B1l_ ()
#4  0x000055c2e41c4e36 in _RNSNvYNCINvMNtCsckbJKWyZekZ_3std6threadNtBa_7Builder16spawn_unchecked_NCNvNtCsbMW2GzUNIV5_9jobserver3imp12spawn_helpers_0uEs_0INtNtN
tCs2vtRKDehYxH_4core3ops8function6FnOnceuE9call_once6vtableB1c_ ()
#5  0x000055c2e42071a3 in std::sys::unix::thread::Thread::new::thread_start ()
#6  0x00007fe3fc0441c7 in start_thread (arg=0x7fe3fb83a700) at pthread_create.c:309
#7  0x00007fe3fb91bb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7fe3fd20b900 (LWP 8640)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055c2e41f9418 in std::sys::unix::futex::futex_wait ()
#2  0x000055c2e41da03c in std::sys::unix::locks::futex_condvar::Condvar::wait_timeout ()
#3  0x000055c2e3d12711 in _RINvMs_NtNtCsckbJKWyZekZ_3std4sync7condvarNtB5_7Condvar18wait_timeout_whileINtNtNtCsMzOSVqp7KB_5cargo4util5queue5StateNtNtNtNtB1i_4c
ore8compiler9job_queue7MessageENCNvMB1e_INtB1e_5QueueB1S_E3pop0EB1i_ ()
#4  0x000055c2e3abec8d in _RNvMNtNtCsMzOSVqp7KB_5cargo4util5queueINtB2_5QueueNtNtNtNtB6_4core8compiler9job_queue7MessageE3popB6_ ()
#5  0x000055c2e3a49aac in _RNvMs3_NtNtNtCsMzOSVqp7KB_5cargo4core8compiler9job_queueNtB5_10DrainState15drain_the_queue ()
#6  0x000055c2e3b6e579 in _RNvXsl_NtNtCs2vtRKDehYxH_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCINvNtNtCsckbJKWyZekZ_3std6thread6scoped5scopeNCNvMs2_NtNt
NtCsMzOSVqp7KB_5cargo4core8compiler9job_queueNtB22_8JobQueue7executes1_0INtNtB9_6result6ResultuNtCs2ZYkk9I22AN_6anyhow5ErrorEE0EINtNtNtB9_3ops8function6FnOnceu
E9call_onceB28_ ()
#7  0x000055c2e3d27133 in _RINvNtNtCsckbJKWyZekZ_3std6thread6scoped5scopeNCNvMs2_NtNtNtCsMzOSVqp7KB_5cargo4core8compiler9job_queueNtBQ_8JobQueue7executes1_0INt
NtCs2vtRKDehYxH_4core6result6ResultuNtCs2ZYkk9I22AN_6anyhow5ErrorEEBW_ ()
#8  0x000055c2e3a46c81 in _RNvMs2_NtNtNtCsMzOSVqp7KB_5cargo4core8compiler9job_queueNtB5_8JobQueue7execute ()
#9  0x000055c2e3e45570 in _RNvMNtNtNtCsMzOSVqp7KB_5cargo4core8compiler7contextNtB2_7Context7compile ()
#10 0x000055c2e39dde66 in _RNvNtNtCsMzOSVqp7KB_5cargo3ops13cargo_compile10compile_ws ()
#11 0x000055c2e39ddb76 in _RNvNtNtCsMzOSVqp7KB_5cargo3ops13cargo_compile7compile ()
#12 0x000055c2e3984968 in _RNvNtNtCs8RuKa4n2wFo_5cargo8commands5rustc4exec ()
#13 0x000055c2e393ca9b in _RNvNtCs8RuKa4n2wFo_5cargo3cli18execute_subcommand ()
#14 0x000055c2e3939e33 in _RNvNtCs8RuKa4n2wFo_5cargo3cli4main ()
#15 0x000055c2e399008f in _RNvCs8RuKa4n2wFo_5cargo4main ()
#16 0x000055c2e39979e3 in _RINvNtNtCsckbJKWyZekZ_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECs8RuKa4n2wFo_5cargo ()
#17 0x000055c2e3924cd9 in _RNCINvNtCsckbJKWyZekZ_3std2rt10lang_startuE0Cs8RuKa4n2wFo_5cargo.llvm.586615937758174569 ()
#18 0x000055c2e41db084 in std::rt::lang_start_internal ()
#19 0x000055c2e3992c95 in main ()
} 9439759625

320968461: gdb -p 26720 {
Thread 2 (Thread 0x7fe2e1414700 (LWP 26721)):
#0  __lll_lock_wait_private () at ../nptl/sysdeps/unix/sysv/linux/x86_64/lowlevellock.S:95
#1  0x00007fe2e74bf41f in _L_lock_9249 () from /lib/libc.so.6
#2  0x00007fe2e74bcea2 in __GI___libc_malloc (bytes=140612330324000) at malloc.c:2884
#3  0x00007fe2ec0f6339 in _dl_map_object_deps (map=0x7fe2ec2e74c8, preloads=<optimized out>, npreloads=<optimized out>, trace_mode=0, open_mode=-2147483648)
    at dl-deps.c:511
#4  0x00007fe2ec0fc4b1 in dl_open_worker (a=0x7fe2ec303950) at dl-open.c:261
#5  0x00007fe2ec0f8362 in _dl_catch_error (objname=0x7fe2ec303998, errstring=0x7fe2ec3039a0, mallocedp=0x7fe2ec3039af, 
    operate=0x7fe2ec0fc3a0 <dl_open_worker>, args=0x7fe2ec303950) at dl-error.c:187
#6  0x00007fe2ec0fbe6a in _dl_open (file=0x7fe2e756ce22 "libgcc_s.so.1", mode=-2147483647, caller_dlopen=0x7fe2e752bb75 <init+21>, nsid=-2, argc=244, 
    argv=<optimized out>, env=0x7ffddaf891e0) at dl-open.c:650
#7  0x00007fe2e7554fb2 in do_dlopen (ptr=0x7fe2ec303b90) at dl-libc.c:87
#8  0x00007fe2ec0f8362 in _dl_catch_error (objname=0x7fe2ec303b68, errstring=0x7fe2ec303b70, mallocedp=0x7fe2ec303b7f, operate=0x7fe2e7554f70 <do_dlopen>, 
    args=0x7fe2ec303b90) at dl-error.c:187
#9  0x00007fe2e755504f in dlerror_run (operate=<optimized out>, args=<optimized out>) at dl-libc.c:46
#10 0x00007fe2e75550c1 in __GI___libc_dlopen_mode (name=<optimized out>, mode=<optimized out>) at dl-libc.c:163
#11 0x00007fe2e752bb75 in init () at ../sysdeps/x86_64/backtrace.c:52
#12 0x00007fe2e2635be3 in pthread_once () at ../nptl/sysdeps/unix/sysv/linux/x86_64/pthread_once.S:103
#13 0x00007fe2e752bc94 in __GI___backtrace (array=<optimized out>, size=256) at ../sysdeps/x86_64/backtrace.c:103
#14 0x00007fe2e823a233 in _RNvNtCs1kQTDMR3aYH_12rustc_driver14signal_handler17print_stack_trace.llvm.7639659949223733406 ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#15 <signal handler called>
#16 0x00007fe2e74b9abd in malloc_consolidate (av=0x7fe2dc000020) at malloc.c:4162
#17 malloc_consolidate (av=0x7fe2dc000020) at malloc.c:4100
#18 0x00007fe2e74ba538 in _int_free (av=0x7fe2dc000020, p=0x7fe2dc0de790, have_lock=0) at malloc.c:4054
#19 0x00007fe2e8f2db5a in _RNvXsi_NtCs7qpKjstCVCm_5alloc2rcINtB5_2RcNtNtNtCsjpyenGmQ8Ns_14rustc_metadata5rmeta7decoder13CrateMetadataENtNtNtCs2vtRKDehYxH_4core
3ops4drop4Drop4dropCsehTlzApPU5J_13rustc_resolve () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#20 0x00007fe2e8f3dc1c in _RINvNtCs2vtRKDehYxH_4core3ptr13drop_in_placeNtNtCsjpyenGmQ8Ns_14rustc_metadata7creader6CStoreECsehTlzApPU5J_13rustc_resolve ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#21 0x00007fe2e822e845 in _RINvNtCs2vtRKDehYxH_4core3ptr13drop_in_placeNtNtCsiRkSCqMMO4G_12rustc_middle2ty18ResolverGlobalCtxtECs1kQTDMR3aYH_12rustc_driver ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#22 0x00007fe2e822de2f in _RINvNtCs2vtRKDehYxH_4core3ptr13drop_in_placeNtNtCs6NFtVQCMgdq_15rustc_interface7queries7QueriesECs1kQTDMR3aYH_12rustc_driver ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#23 0x00007fe2e821947e in _RINvMs2_NtCs6NFtVQCMgdq_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCs1kQTDMR3aYH_12rustc_driver12run_compilers_0
s0_0INtNtCs2vtRKDehYxH_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteedEEB1n_ ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#24 0x00007fe2e81acf6e in _RINvCssZSqRNKEfD_10rustc_span15with_source_mapINtNtCs2vtRKDehYxH_4core6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteed
ENCNCINvNtCs6NFtVQCMgdq_15rustc_interface9interface12run_compilerBI_NCNvCs1kQTDMR3aYH_12rustc_driver12run_compilers_0E0s_0EB3g_ ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#25 0x00007fe2e820753e in _RINvMs_CscMLTBfMV4aa_10scoped_tlsINtB5_9ScopedKeyNtCssZSqRNKEfD_10rustc_span14SessionGlobalsE3setNCINvNtCs6NFtVQCMgdq_15rustc_interf
ace9interface12run_compilerINtNtCs2vtRKDehYxH_4core6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteedENCNvCs1kQTDMR3aYH_12rustc_driver12run_compile
rs_0E0B2x_EB40_ () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#26 0x00007fe2e81ceb7f in _RINvNtNtCsckbJKWyZekZ_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNCINvNtCs6NFtVQCMgdq_15rustc_interface4util31run_in
_thread_pool_with_globalsNCINvNtB1o_9interface12run_compilerINtNtCs2vtRKDehYxH_4core6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteedENCNvCs1kQTDM
R3aYH_12rustc_driver12run_compilers_0E0B34_E00B34_EB4x_ () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#27 0x00007fe2e81b3495 in _RNSNvYNCINvMNtCsckbJKWyZekZ_3std6threadNtBa_7Builder16spawn_unchecked_NCNCINvNtCs6NFtVQCMgdq_15rustc_interface4util31run_in_thread_p
ool_with_globalsNCINvNtB1f_9interface12run_compilerINtNtCs2vtRKDehYxH_4core6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteedENCNvCs1kQTDMR3aYH_12r
ustc_driver12run_compilers_0E0B2V_E00B2V_Es_0INtNtNtB30_3ops8function6FnOnceuE9call_once6vtableB4o_ () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#28 0x00007fe2e7880803 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-c8f87df97cfd2834.so
#29 0x00007fe2e26301c7 in start_thread (arg=0x7fe2e1414700) at pthread_create.c:309
#30 0x00007fe2e751eb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7fe2ec181b80 (LWP 26720)):
#0  0x00007fe2e26316bf in pthread_join (threadid=140612418488064, thread_return=0x0) at pthread_join.c:92
#1  0x00007fe2e7880a6d in std::sys::unix::thread::Thread::join () from /usr/lib/libstd-c8f87df97cfd2834.so
#2  0x00007fe2e81b4f00 in _RNvXsl_NtNtCs2vtRKDehYxH_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCINvNtNtCsckbJKWyZekZ_3std6thread6scoped5scopeNCINvNtCs6NF
tVQCMgdq_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB21_9interface12run_compilerINtNtB9_6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15Error
GuaranteedENCNvCs1kQTDMR3aYH_12rustc_driver12run_compilers_0E0B3H_E0B3H_E0EINtNtNtB9_3ops8function6FnOnceuE9call_onceB4U_ ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#3  0x00007fe2e81b2493 in _RINvNtNtCsckbJKWyZekZ_3std6thread6scoped5scopeNCINvNtCs6NFtVQCMgdq_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtBP_
9interface12run_compilerINtNtCs2vtRKDehYxH_4core6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteedENCNvCs1kQTDMR3aYH_12rustc_driver12run_compilers_
0E0B2u_E0B2u_EB3X_ () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#4  0x00007fe2e820cc7d in _RINvNtCs6NFtVQCMgdq_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB4_9interface12run_compilerINtNtCs2vtRKDehYxH_4cor
e6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15ErrorGuaranteedENCNvCs1kQTDMR3aYH_12rustc_driver12run_compilers_0E0B1J_EB3c_ ()
   from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#5  0x00007fe2e8236e49 in _RINvNtCs6NFtVQCMgdq_15rustc_interface9interface12run_compilerINtNtCs2vtRKDehYxH_4core6result6ResultuNtCshxgXFo2ElGW_12rustc_errors15
ErrorGuaranteedENCNvCs1kQTDMR3aYH_12rustc_driver12run_compilers_0EB2q_ () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#6  0x00007fe2e823f9b3 in _RNvMs_Cs1kQTDMR3aYH_12rustc_driverNtB4_11RunCompiler3run () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#7  0x00007fe2e81b57fb in _RNvXsl_NtNtCs2vtRKDehYxH_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCNvCs1kQTDMR3aYH_12rustc_driver4main0EINtNtNtB9_3ops8funct
ion6FnOnceuE9call_onceB1d_ () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#8  0x00007fe2e824723c in _RNvCs1kQTDMR3aYH_12rustc_driver4main () from /usr/lib/librustc_driver-3bbe95a033bd6c07.so
#9  0x000055ccf9c00847 in _RNvCs8s96E1WGSZP_10rustc_main4main ()
#10 0x000055ccf9c00883 in _RINvNtNtCsckbJKWyZekZ_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECs8s96E1WGSZP_10rustc_main ()
#11 0x000055ccf9c008a9 in _RNCINvNtCsckbJKWyZekZ_3std2rt10lang_startuE0Cs8s96E1WGSZP_10rustc_main.llvm.14244257621252912080 ()
---Type <return> to continue, or q <return> to quit---
#12 0x00007fe2e783cc14 in std::rt::lang_start_internal () from /usr/lib/libstd-c8f87df97cfd2834.so
#13 0x000055ccf9c00878 in main ()
} 320968461

