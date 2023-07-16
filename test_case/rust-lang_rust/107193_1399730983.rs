
2870366926 stack trace for threads {

make(21883)───cargo(21906)─┬─rustc(12396)─┬─{rustc}(12397)
                           │              ├─{rustc}(12404)
                           │              ├─{rustc}(12405)
                           │              └─{rustc}(12406)
                           ├─{cargo}(21921)
                           └─{cargo}(12395)
  PID   LWP
21906 21906
21906 21921
21906 12395

2281567429: gdb -p 21906 {

Thread 3 (Thread 0x7fb86d1e8700 (LWP 12395)):
#0  0x00007fb8d4ca4ffd in poll () at ../sysdeps/unix/syscall-template.S:81
#1  0x000055ca299c8dce in _RNvNtNtCsdZO6ukqL1Yw_10cargo_util5read23imp5read2 ()
#2  0x000055ca299c5fdc in _RNvMs_NtCsdZO6ukqL1Yw_10cargo_util15process_builderNtB4_14ProcessBuilder19exec_with_streaming ()
#3  0x000055ca295a61ed in _RNvXs_NtNtCsaNPHh7OdIlO_5cargo4core8compilerNtB4_15DefaultExecutorNtB4_8Executor4exec ()
#4  0x000055ca292e784e in _RNSNvYNCNvNtNtCsaNPHh7OdIlO_5cargo4core8compiler5rustcs0_0INtNtNtCshHjhugwbahS_4core3ops8function6FnOnceTRNtNtB8_9job_queue8JobState
EE9call_once6vtableBc_.llvm.5925800752365715571 ()
#5  0x000055ca292e5fac in _RNSNvYNCNvMNtNtNtCsaNPHh7OdIlO_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCshHjhugwbahS_4core3ops8function6FnOnceTRNtNtBb_9job_q
ueue8JobStateEE9call_once6vtableBf_.llvm.5925800752365715571 ()
#6  0x000055ca292e5fac in _RNSNvYNCNvMNtNtNtCsaNPHh7OdIlO_5cargo4core8compiler3jobNtB9_4Work4then0INtNtNtCshHjhugwbahS_4core3ops8function6FnOnceTRNtNtBb_9job_q
ueue8JobStateEE9call_once6vtableBf_.llvm.5925800752365715571 ()
#7  0x000055ca2921e636 in _RNSNvYNCINvMs1_NtCsja0OYazk36h_15crossbeam_utils6threadNtBd_19ScopedThreadBuilder5spawnNCNvMs3_NtNtNtCsaNPHh7OdIlO_5cargo4core8compi
ler9job_queueNtB1v_10DrainState3runs_0uE0INtNtNtCshHjhugwbahS_4core3ops8function6FnOnceuE9call_once6vtableB1B_ ()
#8  0x000055ca292d775e in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceINtNtCsdsMurdI0nrl_5alloc5boxed3BoxDINtNtNtCshHjhugwb
ahS_4core3ops8function6FnOnceuEp6OutputuNtNtB1W_6marker4SendEL_EuECsaNPHh7OdIlO_5cargo ()
#9  0x000055ca2959ac7a in _RNSNvYNCINvMNtCskiJB1pEBOHb_3std6threadNtBa_7Builder16spawn_unchecked_INtNtCsdsMurdI0nrl_5alloc5boxed3BoxDINtNtNtCshHjhugwbahS_4core
3ops8function6FnOnceuEp6OutputuNtNtB1N_6marker4SendEL_EuEs_0IB1H_uE9call_once6vtableCsaNPHh7OdIlO_5cargo ()
#10 0x000055ca299efd03 in std::sys::unix::thread::Thread::new::thread_start ()
#11 0x00007fb8d53d61c7 in start_thread (arg=0x7fb86d1e8700) at pthread_create.c:309
#12 0x00007fb8d4cadb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 2 (Thread 0x7fb8d4bcc700 (LWP 21921)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055ca299e9938 in std::sys::unix::futex::futex_wait ()
#2  0x000055ca299ec8a7 in std::sys::unix::locks::futex_condvar::Condvar::wait ()
#3  0x000055ca299de1ef in _RINvMs3_Cs8ROwNbzQDim_9jobserverNtB6_11HelperState16for_each_requestNCNCNvNtB6_3imp12spawn_helpers_00EB6_ ()
#4  0x000055ca299deaec in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvNtCs8ROwNbzQDim_9jobserver3imp12spawn_helpers_0uE
B1l_.llvm.17672469238476870296 ()
#5  0x000055ca299df7af in _RNSNvYNCINvMNtCskiJB1pEBOHb_3std6threadNtBa_7Builder16spawn_unchecked_NCNvNtCs8ROwNbzQDim_9jobserver3imp12spawn_helpers_0uEs_0INtNtN
tCshHjhugwbahS_4core3ops8function6FnOnceuE9call_once6vtableB1c_ ()
#6  0x000055ca299efd03 in std::sys::unix::thread::Thread::new::thread_start ()
#7  0x00007fb8d53d61c7 in start_thread (arg=0x7fb8d4bcc700) at pthread_create.c:309
#8  0x00007fb8d4cadb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7fb8d659d900 (LWP 21906)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x000055ca299e9938 in std::sys::unix::futex::futex_wait ()
#2  0x000055ca299ec931 in std::sys::unix::locks::futex_condvar::Condvar::wait_timeout ()
#3  0x000055ca2950a65e in _RINvMs_NtNtCskiJB1pEBOHb_3std4sync7condvarNtB5_7Condvar18wait_timeout_whileINtNtNtCsaNPHh7OdIlO_5cargo4util5queue5StateNtNtNtNtB1i_4
core8compiler9job_queue7MessageENCNvMB1e_INtB1e_5QueueB1T_E3pop0EB1i_ ()
#4  0x000055ca2931b44d in _RNvMNtNtCsaNPHh7OdIlO_5cargo4util5queueINtB2_5QueueNtNtNtNtB6_4core8compiler9job_queue7MessageE3popB6_ ()
#5  0x000055ca293c6974 in _RNvMs3_NtNtNtCsaNPHh7OdIlO_5cargo4core8compiler9job_queueNtB5_10DrainState15drain_the_queue ()
#6  0x000055ca295a1279 in _RNvXsl_NtNtCshHjhugwbahS_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCINvNtCsja0OYazk36h_15crossbeam_utils6thread5scopeNCNvMs2_
NtNtNtCsaNPHh7OdIlO_5cargo4core8compiler9job_queueNtB26_8JobQueue7executes1_0INtNtB9_6result6ResultuNtCsdf2yQx6j5xH_6anyhow5ErrorEE0EINtNtNtB9_3ops8function6Fn
OnceuE9call_onceB2c_ ()
#7  0x000055ca291f203c in _RINvNtCsja0OYazk36h_15crossbeam_utils6thread5scopeNCNvMs2_NtNtNtCsaNPHh7OdIlO_5cargo4core8compiler9job_queueNtBU_8JobQueue7executes1
_0INtNtCshHjhugwbahS_4core6result6ResultuNtCsdf2yQx6j5xH_6anyhow5ErrorEEB10_ ()
#8  0x000055ca293c551a in _RNvMs2_NtNtNtCsaNPHh7OdIlO_5cargo4core8compiler9job_queueNtB5_8JobQueue7execute ()
#9  0x000055ca292f87cd in _RNvMNtNtNtCsaNPHh7OdIlO_5cargo4core8compiler7contextNtB2_7Context7compile ()
#10 0x000055ca295c1620 in _RNvNtNtCsaNPHh7OdIlO_5cargo3ops13cargo_compile10compile_ws ()
#11 0x000055ca295c1386 in _RNvNtNtCsaNPHh7OdIlO_5cargo3ops13cargo_compile7compile ()
#12 0x000055ca291a3262 in _RNvNtNtCscQn2M28fF5X_5cargo8commands5rustc4exec ()
#13 0x000055ca291940bb in _RNvNtCscQn2M28fF5X_5cargo3cli18execute_subcommand ()
#14 0x000055ca29191611 in _RNvNtCscQn2M28fF5X_5cargo3cli4main ()
#15 0x000055ca29161c3d in _RNvCscQn2M28fF5X_5cargo4main ()
#16 0x000055ca291ae4f3 in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECscQn2M28fF5X_5cargo ()
#17 0x000055ca291aec49 in _RNCINvNtCskiJB1pEBOHb_3std2rt10lang_startuE0CscQn2M28fF5X_5cargo.llvm.6965099604065319556 ()
#18 0x000055ca299ebcf5 in std::rt::lang_start_internal ()
#19 0x000055ca29163da2 in main ()

} 2281567429

94055202: gdb -p 12396 {

Thread 5 (Thread 0x7f46b0b96700 (LWP 12406)):
#0  __lll_lock_wait_private () at ../nptl/sysdeps/unix/sysv/linux/x86_64/lowlevellock.S:95
#1  0x00007f46e68b0119 in _L_lock_3329 () from /lib/libc.so.6
#2  0x00007f46e68ab3db in _int_free (av=0x7f46dc000020, p=0x7f46dd81ee10, have_lock=0) at malloc.c:3940
#3  0x00007f46e2a5822d in llvm::Instruction::eraseFromParent() () from /usr/lib/libLLVM-13.so
#4  0x00007f46e36238ed in llvm::InlineFunction(llvm::CallBase&, llvm::InlineFunctionInfo&, llvm::AAResults*, bool, llvm::Function*) ()
   from /usr/lib/libLLVM-13.so
#5  0x00007f46e3d23a43 in llvm::InlinerPass::run(llvm::LazyCallGraph::SCC&, llvm::AnalysisManager<llvm::LazyCallGraph::SCC, llvm::LazyCallGraph&>&, llvm::LazyC
allGraph&, llvm::CGSCCUpdateResult&) [clone .localalias] () from /usr/lib/libLLVM-13.so
#6  0x00007f46e3d24e4e in llvm::detail::PassModel<llvm::LazyCallGraph::SCC, llvm::InlinerPass, llvm::PreservedAnalyses, llvm::AnalysisManager<llvm::LazyCallGra
ph::SCC, llvm::LazyCallGraph&>, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&>::run(llvm::LazyCallGraph::SCC&, llvm::AnalysisManager<llvm::LazyCallGraph::SCC,
 llvm::LazyCallGraph&>&, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&) () from /usr/lib/libLLVM-13.so
#7  0x00007f46e3fbabb4 in llvm::PassManager<llvm::LazyCallGraph::SCC, llvm::AnalysisManager<llvm::LazyCallGraph::SCC, llvm::LazyCallGraph&>, llvm::LazyCallGrap
h&, llvm::CGSCCUpdateResult&>::run(llvm::LazyCallGraph::SCC&, llvm::AnalysisManager<llvm::LazyCallGraph::SCC, llvm::LazyCallGraph&>&, llvm::LazyCallGraph&, llv
m::CGSCCUpdateResult&) () from /usr/lib/libLLVM-13.so
#8  0x00007f46e3d1a0de in llvm::detail::PassModel<llvm::LazyCallGraph::SCC, llvm::PassManager<llvm::LazyCallGraph::SCC, llvm::AnalysisManager<llvm::LazyCallGra
ph::SCC, llvm::LazyCallGraph&>, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&>, llvm::PreservedAnalyses, llvm::AnalysisManager<llvm::LazyCallGraph::SCC, llvm:
:LazyCallGraph&>, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&>::run(llvm::LazyCallGraph::SCC&, llvm::AnalysisManager<llvm::LazyCallGraph::SCC, llvm::LazyCal
lGraph&>&, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&) () from /usr/lib/libLLVM-13.so
#9  0x00007f46e3fc0c41 in llvm::DevirtSCCRepeatedPass::run(llvm::LazyCallGraph::SCC&, llvm::AnalysisManager<llvm::LazyCallGraph::SCC, llvm::LazyCallGraph&>&, l
lvm::LazyCallGraph&, llvm::CGSCCUpdateResult&) () from /usr/lib/libLLVM-13.so
#10 0x00007f46e3d1a0be in llvm::detail::PassModel<llvm::LazyCallGraph::SCC, llvm::DevirtSCCRepeatedPass, llvm::PreservedAnalyses, llvm::AnalysisManager<llvm::L
azyCallGraph::SCC, llvm::LazyCallGraph&>, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&>::run(llvm::LazyCallGraph::SCC&, llvm::AnalysisManager<llvm::LazyCallG
raph::SCC, llvm::LazyCallGraph&>&, llvm::LazyCallGraph&, llvm::CGSCCUpdateResult&) () from /usr/lib/libLLVM-13.so
#11 0x00007f46e3fbbfab in llvm::ModuleToPostOrderCGSCCPassAdaptor::run(llvm::Module&, llvm::AnalysisManager<llvm::Module>&) () from /usr/lib/libLLVM-13.so
#12 0x00007f46e3d1d40a in llvm::ModuleInlinerWrapperPass::run(llvm::Module&, llvm::AnalysisManager<llvm::Module>&) () from /usr/lib/libLLVM-13.so
#13 0x00007f46e506c75e in llvm::detail::PassModel<llvm::Module, llvm::ModuleInlinerWrapperPass, llvm::PreservedAnalyses, llvm::AnalysisManager<llvm::Module>>::
run(llvm::Module&, llvm::AnalysisManager<llvm::Module>&) () from /usr/lib/libLLVM-13.so
#14 0x00007f46e2ad489b in llvm::PassManager<llvm::Module, llvm::AnalysisManager<llvm::Module>>::run(llvm::Module&, llvm::AnalysisManager<llvm::Module>&) ()
   from /usr/lib/libLLVM-13.so
#15 0x00007f46e79065df in LLVMRustOptimizeWithNewPassManager () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#16 0x00007f46e78c12f3 in _RNvNtNtCsgEjqgbllu38_18rustc_codegen_llvm4back5write35optimize_with_new_llvm_pass_manager ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#17 0x00007f46e78c17d5 in _RNvNtNtCsgEjqgbllu38_18rustc_codegen_llvm4back5write8optimize () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#18 0x00007f46e77c26e4 in _RINvNtNtCs3DQi51F8lXU_17rustc_codegen_ssa4back5write17execute_work_itemNtCsgEjqgbllu38_18rustc_codegen_llvm18LlvmCodegenBackendEB19_
 () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#19 0x00007f46e77a52f9 in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvXs0_CsgEjqgbllu38_18rustc_codegen_llvmNtB1o_18Ll
vmCodegenBackendNtNtNtCs3DQi51F8lXU_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2s_4back5write10spawn_workB1W_E0uE0uEB
1o_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#20 0x00007f46e788aeb7 in _RNSNvYNCINvMNtCskiJB1pEBOHb_3std6threadNtBa_7Builder16spawn_unchecked_NCINvXs0_CsgEjqgbllu38_18rustc_codegen_llvmNtB1f_18LlvmCodegen
BackendNtNtNtCs3DQi51F8lXU_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2j_4back5write10spawn_workB1N_E0uE0uEs_0INtNtNt
CshHjhugwbahS_4core3ops8function6FnOnceuE9call_once6vtableB1f_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#21 0x00007f46e6c228f3 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-e1a984d4bb4b678c.so
#22 0x00007f46e1a211c7 in start_thread (arg=0x7f46b0b96700) at pthread_create.c:309
#23 0x00007f46e690fb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 4 (Thread 0x7f46b0d97700 (LWP 12405)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x00007f46e6c13b38 in std::sys::unix::futex::futex_wait () from /usr/lib/libstd-e1a984d4bb4b678c.so
#2  0x00007f46e6c170b9 in std::thread::park () from /usr/lib/libstd-e1a984d4bb4b678c.so
#3  0x00007f46e6c3ab72 in std::sync::mpsc::blocking::WaitToken::wait () from /usr/lib/libstd-e1a984d4bb4b678c.so
#4  0x00007f46e788b7b7 in _RNvMNtNtNtCskiJB1pEBOHb_3std4sync4mpsc6sharedINtB2_6PacketINtNtCsdsMurdI0nrl_5alloc5boxed3BoxDNtNtCshHjhugwbahS_4core3any3AnyNtNtB1y
_6marker4SendEL_EE4recvCsgEjqgbllu38_18rustc_codegen_llvm () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#5  0x00007f46e7819921 in _RNvMsd_NtNtCskiJB1pEBOHb_3std4sync4mpscINtB5_8ReceiverINtNtCsdsMurdI0nrl_5alloc5boxed3BoxDNtNtCshHjhugwbahS_4core3any3AnyNtNtB1u_6ma
rker4SendEL_EE4recvCsgEjqgbllu38_18rustc_codegen_llvm () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#6  0x00007f46e77a2be0 in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvXs0_CsgEjqgbllu38_18rustc_codegen_llvmNtB1o_18Ll
vmCodegenBackendNtNtNtCs3DQi51F8lXU_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods12spawn_threadNCINvNtNtB2s_4back5write20start_executing_workB1W_Es2_
0INtNtCshHjhugwbahS_4core6result6ResultNtB3S_15CompiledModulesuEE0B4G_EB1o_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#7  0x00007f46e788abf9 in _RNSNvYNCINvMNtCskiJB1pEBOHb_3std6threadNtBa_7Builder16spawn_unchecked_NCINvXs0_CsgEjqgbllu38_18rustc_codegen_llvmNtB1f_18LlvmCodegen
BackendNtNtNtCs3DQi51F8lXU_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods12spawn_threadNCINvNtNtB2j_4back5write20start_executing_workB1N_Es2_0INtNtCsh
HjhugwbahS_4core6result6ResultNtB3J_15CompiledModulesuEE0B4x_Es_0INtNtNtB4C_3ops8function6FnOnceuE9call_once6vtableB1f_ ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#8  0x00007f46e6c228f3 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-e1a984d4bb4b678c.so
#9  0x00007f46e1a211c7 in start_thread (arg=0x7f46b0d97700) at pthread_create.c:309
#10 0x00007f46e690fb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 3 (Thread 0x7f46dbfff700 (LWP 12404)):
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x00007f46e6c13b38 in std::sys::unix::futex::futex_wait () from /usr/lib/libstd-e1a984d4bb4b678c.so
#2  0x00007f46e6c1bcd7 in std::sys::unix::locks::futex_condvar::Condvar::wait () from /usr/lib/libstd-e1a984d4bb4b678c.so
#3  0x00007f46ea23fa1f in _RINvMs3_CsgII5ob7RKqm_9jobserverNtB6_11HelperState16for_each_requestNCNCNvNtB6_3imp12spawn_helpers_00EB6_ ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#4  0x00007f46ea24085c in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNvNtCsgII5ob7RKqm_9jobserver3imp12spawn_helpers_0uE
B1l_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
---Type <return> to continue, or q <return> to quit---
#5  0x00007f46ea24180f in _RNSNvYNCINvMNtCskiJB1pEBOHb_3std6threadNtBa_7Builder16spawn_unchecked_NCNvNtCsgII5ob7RKqm_9jobserver3imp12spawn_helpers_0uEs_0INtNtN
tCshHjhugwbahS_4core3ops8function6FnOnceuE9call_once6vtableB1c_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#6  0x00007f46e6c228f3 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-e1a984d4bb4b678c.so
#7  0x00007f46e1a211c7 in start_thread (arg=0x7f46dbfff700) at pthread_create.c:309
#8  0x00007f46e690fb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 2 (Thread 0x7f46e0805700 (LWP 12397)):
#0  __lll_lock_wait_private () at ../nptl/sysdeps/unix/sysv/linux/x86_64/lowlevellock.S:95
#1  0x00007f46e68b041f in _L_lock_9249 () at malloc.c:5175
#2  0x00007f46e68adea2 in __GI___libc_malloc (bytes=139942315425824) at malloc.c:2884
#3  0x00007f46eb3c5339 in _dl_map_object_deps (map=0x7f46eb5b64c8, preloads=<optimized out>, npreloads=<optimized out>, trace_mode=0, open_mode=-2147483648)
    at dl-deps.c:511
#4  0x00007f46eb3cb4b1 in dl_open_worker (a=0x7f46eb5d2950) at dl-open.c:261
#5  0x00007f46eb3c7362 in _dl_catch_error (objname=0x7f46eb5d2998, errstring=0x7f46eb5d29a0, mallocedp=0x7f46eb5d29af, 
    operate=0x7f46eb3cb3a0 <dl_open_worker>, args=0x7f46eb5d2950) at dl-error.c:187
#6  0x00007f46eb3cae6a in _dl_open (file=0x7f46e695de22 "libgcc_s.so.1", mode=-2147483647, caller_dlopen=0x7f46e691cb75 <init+21>, nsid=-2, argc=244, 
    argv=<optimized out>, env=0x7fff15a552d0) at dl-open.c:650
#7  0x00007f46e6945fb2 in do_dlopen (ptr=0x7f46eb5d2b90) at dl-libc.c:87
#8  0x00007f46eb3c7362 in _dl_catch_error (objname=0x7f46eb5d2b68, errstring=0x7f46eb5d2b70, mallocedp=0x7f46eb5d2b7f, operate=0x7f46e6945f70 <do_dlopen>, 
    args=0x7f46eb5d2b90) at dl-error.c:187
#9  0x00007f46e694604f in dlerror_run (operate=<optimized out>, args=<optimized out>) at dl-libc.c:46
#10 0x00007f46e69460c1 in __GI___libc_dlopen_mode (name=<optimized out>, mode=<optimized out>) at dl-libc.c:163
#11 0x00007f46e691cb75 in init () at ../sysdeps/x86_64/backtrace.c:52
#12 0x00007f46e1a26be3 in pthread_once () at ../nptl/sysdeps/unix/sysv/linux/x86_64/pthread_once.S:103
#13 0x00007f46e691cc94 in __GI___backtrace (array=<optimized out>, size=256) at ../sysdeps/x86_64/backtrace.c:103
#14 0x00007f46e7612373 in _RNvNtCs2Jf8BZxvLdB_12rustc_driver14signal_handler17print_stack_trace.llvm.17198381433968885060 ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#15 <signal handler called>
#16 0x00007f46e68aaabd in malloc_consolidate (av=0x7f46dc000020) at malloc.c:4162
#17 malloc_consolidate (av=0x7f46dc000020) at malloc.c:4100
#18 0x00007f46e68ab538 in _int_free (av=0x7f46dc000020, p=0x7f46dc196940, have_lock=0) at malloc.c:4054
#19 0x00007f46e836415a in _RNvXsi_NtCsdsMurdI0nrl_5alloc2rcINtB5_2RcNtNtNtCsfZO9feTNX4O_14rustc_metadata5rmeta7decoder13CrateMetadataENtNtNtCshHjhugwbahS_4core
3ops4drop4Drop4dropCs5UaALbvq2EP_13rustc_resolve () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#20 0x00007f46e82d4ebd in _RINvNtCshHjhugwbahS_4core3ptr13drop_in_placeNtNtCsfZO9feTNX4O_14rustc_metadata7creader6CStoreECs5UaALbvq2EP_13rustc_resolve ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#21 0x00007f46e75c0f53 in _RINvNtCshHjhugwbahS_4core3ptr13drop_in_placeNtNtCsfRHOU8ZuVRc_15rustc_interface7queries7QueriesECs2Jf8BZxvLdB_12rustc_driver ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#22 0x00007f46e75a8c8b in _RINvMs2_NtCsfRHOU8ZuVRc_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCs2Jf8BZxvLdB_12rustc_driver12run_compilers_0
s0_0INtNtCshHjhugwbahS_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuaranteedEEB1n_ ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#23 0x00007f46e760c2e1 in _RINvCsjXzf5WQcbm7_10rustc_span15with_source_mapINtNtCshHjhugwbahS_4core6result6ResultuNtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuarantee
dENCINvNtCsfRHOU8ZuVRc_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCs2Jf8BZxvLdB_12rustc_driver12run_compilers_0Es_0EB3q_ ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#24 0x00007f46e75a94ca in _RINvMs_Csdvxop7xbmVJ_10scoped_tlsINtB5_9ScopedKeyNtCsjXzf5WQcbm7_10rustc_span14SessionGlobalsE3setNCINvNtCsfRHOU8ZuVRc_15rustc_inter
face9interface12run_compilerINtNtCshHjhugwbahS_4core6result6ResultuNtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuaranteedENCNvCs2Jf8BZxvLdB_12rustc_driver12run_compil
ers_0E0B2y_EB41_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#25 0x00007f46e75ce85f in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCsfRHOU8ZuVRc_15rustc_interface4util31run_in_t
hread_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCshHjhugwbahS_4core6result6ResultuNtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuaranteedENCNvCs2Jf8BZxv
LdB_12rustc_driver12run_compilers_0E0B32_E0B32_EB4v_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#26 0x00007f46e75cf5f9 in _RNSNvYNCINvMNtCskiJB1pEBOHb_3std6threadNtBa_7Builder16spawn_unchecked_NCINvNtCsfRHOU8ZuVRc_15rustc_interface4util31run_in_thread_poo
l_with_globalsNCINvNtB1d_9interface12run_compilerINtNtCshHjhugwbahS_4core6result6ResultuNtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuaranteedENCNvCs2Jf8BZxvLdB_12rus
tc_driver12run_compilers_0E0B2T_E0B2T_Es_0INtNtNtB2Y_3ops8function6FnOnceuE9call_once6vtableB4m_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#27 0x00007f46e6c228f3 in std::sys::unix::thread::Thread::new::thread_start () from /usr/lib/libstd-e1a984d4bb4b678c.so
#28 0x00007f46e1a211c7 in start_thread (arg=0x7f46e0805700) at pthread_create.c:309
#29 0x00007f46e690fb6d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111

Thread 1 (Thread 0x7f46eb450bc0 (LWP 12396)):
#0  0x00007f46e1a226bf in pthread_join (threadid=139942390945536, thread_return=0x0) at pthread_join.c:92
#1  0x00007f46e6c22aad in std::sys::unix::thread::Thread::join () from /usr/lib/libstd-e1a984d4bb4b678c.so
#2  0x00007f46e75cfe54 in _RNvMs8_NtCskiJB1pEBOHb_3std6threadINtB5_10JoinHandleINtNtCshHjhugwbahS_4core6result6ResultuNtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuar
anteedEE4joinCs2Jf8BZxvLdB_12rustc_driver () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#3  0x00007f46e758bcfe in _RINvNtCsfRHOU8ZuVRc_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB4_9interface12run_compilerINtNtCshHjhugwbahS_4cor
e6result6ResultuNtCs1f0AvXLQ7xU_12rustc_errors15ErrorGuaranteedENCNvCs2Jf8BZxvLdB_12rustc_driver12run_compilers_0E0B1J_EB3c_ ()
   from /usr/lib/librustc_driver-9574be8af1aa584f.so
#4  0x00007f46e76206e8 in _RNvMs_Cs2Jf8BZxvLdB_12rustc_driverNtB4_11RunCompiler3run () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#5  0x00007f46e7592b47 in _RNvXsl_NtNtCshHjhugwbahS_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCNvCs2Jf8BZxvLdB_12rustc_driver4main0EINtNtNtB9_3ops8funct
ion6FnOnceuE9call_onceB1d_ () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#6  0x00007f46e7627c77 in _RNvCs2Jf8BZxvLdB_12rustc_driver4main () from /usr/lib/librustc_driver-9574be8af1aa584f.so
#7  0x0000563cf5c008bd in _RNvCs9FduDbT7m4B_10rustc_main4main ()
#8  0x0000563cf5c008f3 in _RINvNtNtCskiJB1pEBOHb_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECs9FduDbT7m4B_10rustc_main ()
#9  0x0000563cf5c00919 in _RNCINvNtCskiJB1pEBOHb_3std2rt10lang_startuE0Cs9FduDbT7m4B_10rustc_main.llvm.7915475400312840929 ()
#10 0x00007f46e6c1a285 in std::rt::lang_start_internal () from /usr/lib/libstd-e1a984d4bb4b678c.so
#11 0x0000563cf5c008e2 in main ()

} 94055202

} 2870366926

