
% gdb ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc
GNU gdb (GDB) Fedora 8.0.1-36.fc27
Copyright (C) 2017 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-redhat-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/bin/rustc.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) r -Clto ../src/test/ui/issues/issue-44056.rs
Starting program: /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Clto ../src/test/ui/issues/issue-44056.rs
warning: the debug information found in "/usr/lib/debug//lib64/ld-2.26.so.debug" does not match "/lib64/ld-linux-x86-64.so.2" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//usr/lib64/ld-2.26.so.debug" does not match "/lib64/ld-linux-x86-64.so.2" (CRC mismatch).

Missing separate debuginfos, use: dnf debuginfo-install glibc-2.26-24.fc27.x86_64
warning: the debug information found in "/usr/lib/debug//lib64/libc-2.26.so.debug" does not match "/lib64/libc.so.6" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//usr/lib64/libc-2.26.so.debug" does not match "/lib64/libc.so.6" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//lib64/libdl-2.26.so.debug" does not match "/lib64/libdl.so.2" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//usr/lib64/libdl-2.26.so.debug" does not match "/lib64/libdl.so.2" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//lib64/libpthread-2.26.so.debug" does not match "/lib64/libpthread.so.0" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//usr/lib64/libpthread-2.26.so.debug" does not match "/lib64/libpthread.so.0" (CRC mismatch).

[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".
warning: the debug information found in "/usr/lib/debug//lib64/librt-2.26.so.debug" does not match "/lib64/librt.so.1" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//usr/lib64/librt-2.26.so.debug" does not match "/lib64/librt.so.1" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//lib64/libm-2.26.so.debug" does not match "/lib64/libm.so.6" (CRC mismatch).

warning: the debug information found in "/usr/lib/debug//usr/lib64/libm-2.26.so.debug" does not match "/lib64/libm.so.6" (CRC mismatch).

[New Thread 0x7fffe6d94700 (LWP 26218)]
[New Thread 0x7fffe6b93700 (LWP 26219)]
[New Thread 0x7fffe6992700 (LWP 26220)]
[Thread 0x7fffe6992700 (LWP 26220) exited]
[New Thread 0x7fffe6791700 (LWP 26221)]
[Thread 0x7fffe6791700 (LWP 26221) exited]
[New Thread 0x7fffe6791700 (LWP 26222)]
[Thread 0x7fffe6791700 (LWP 26222) exited]
[New Thread 0x7fffe6791700 (LWP 26223)]
[Thread 0x7fffe6791700 (LWP 26223) exited]
[New Thread 0x7fffe6791700 (LWP 26224)]
[Thread 0x7fffe6791700 (LWP 26224) exited]
[New Thread 0x7fffe6791700 (LWP 26225)]
[Thread 0x7fffe6791700 (LWP 26225) exited]
[New Thread 0x7fffe6791700 (LWP 26226)]
[Thread 0x7fffe6791700 (LWP 26226) exited]
[New Thread 0x7fffe6791700 (LWP 26227)]

Thread 11 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffe6791700 (LWP 26227)]
0x00007fffed41e94c in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
Missing separate debuginfos, use: dnf debuginfo-install libgcc-7.3.1-2.fc27.x86_64 libstdc++-7.3.1-2.fc27.x86_64
(gdb) bt
#0  0x00007fffed41e94c in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fffed41eba8 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fffed41ebf4 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fffed4207f1 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fffed41e700 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fffed41eba8 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fffed41ebf4 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fffed4207f1 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffed41e700 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffed41eba8 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffed41ebf4 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fffed3f666e in llvm::DwarfCompileUnit::getOrCreateGlobalVariableDIE(llvm::DIGlobalVariable const*, llvm::ArrayRef<llvm::DwarfCompileUnit::GlobalExpr>) ()
   from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fffed40d92d in llvm::DwarfDebug::beginModule() () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffed3d3815 in llvm::AsmPrinter::doInitialization(llvm::Module&) () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fffedf33ff6 in llvm::FPPassManager::doInitialization(llvm::Module&) () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fffedf3e48e in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fffec4df26f in LLVMRustWriteOutputFile () from /home/pnkfelix/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007fffec2ebcaa in rustc_codegen_llvm::back::write::write_output_file (handler=0x7fffe678fef0, target=0x0, pm=0x19, m=0x7fffdc0868d0, output=0x7fffd1748eb0, file_type=rustc_codegen_llvm::llvm::ffi::ObjectFile) at librustc_codegen_llvm/back/write.rs:109
#18 0x00007fffec41e539 in rustc_codegen_llvm::back::write::codegen::{{closure}}::{{closure}} (cpm=0x7fffd841bd70) at librustc_codegen_llvm/back/write.rs:831
#19 rustc_codegen_llvm::back::write::codegen::with_codegen (tm=<optimized out>, llmod=<optimized out>, f=..., no_builtins=<optimized out>) at librustc_codegen_llvm/back/write.rs:711
#20 rustc_codegen_llvm::back::write::codegen::{{closure}} () at librustc_codegen_llvm/back/write.rs:830
#21 0x00007fffec41d1cb in rustc::util::common::time_ext (do_it=<optimized out>, sess=..., what=..., f=...) at /home/pnkfelix/Dev/Mozilla/rust.git/src/librustc/util/common.rs:163
#22 0x00007fffec2ee655 in rustc_codegen_llvm::back::write::codegen (cgcx=<optimized out>, diag_handler=<optimized out>, module=..., config=<optimized out>, timeline=<optimized out>) at librustc_codegen_llvm/back/write.rs:767
#23 0x00007fffec2f4f1b in rustc_codegen_llvm::back::write::execute_lto_work_item (cgcx=<optimized out>, module_config=0x555555c62300, timeline=<optimized out>, module=...) at librustc_codegen_llvm/back/write.rs:1486
#24 rustc_codegen_llvm::back::write::execute_work_item (cgcx=0x7fffe6790298, work_item=..., timeline=0x7fffe6790140) at librustc_codegen_llvm/back/write.rs:1337
#25 0x00007fffec368558 in rustc_codegen_llvm::back::write::spawn_work::{{closure}} () at librustc_codegen_llvm/back/write.rs:2145
#26 std::sys_common::backtrace::__rust_begin_short_backtrace (f=...) at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/sys_common/backtrace.rs:136
#27 0x00007fffec2dee58 in std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}} () at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/thread/mod.rs:477
#28 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once (self=..., _args=<optimized out>) at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/panic.rs:313
#29 std::panicking::try::do_call (data=<optimized out>) at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/panicking.rs:310
#30 0x00007ffff768000a in __rust_maybe_catch_panic (f=0x7fffd91be010, data=0x0, data_ptr=0x7fffe6790638, vtable_ptr=0x7fffe6790640) at libpanic_unwind/lib.rs:102
#31 0x00007fffec2decdb in std::panicking::try (f=...) at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/panicking.rs:289
#32 0x00007fffec3e027e in std::panic::catch_unwind (f=...) at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/panic.rs:392
#33 std::thread::Builder::spawn_unchecked::{{closure}} () at /home/pnkfelix/Dev/Mozilla/rust.git/src/libstd/thread/mod.rs:476
#34 <F as alloc::boxed::FnBox<A>>::call_box (self=0x7fffd0f64e70, args=<optimized out>) at /home/pnkfelix/Dev/Mozilla/rust.git/src/liballoc/boxed.rs:672
#35 0x00007ffff766827b in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h621530a0cd50b060 (self=..., args=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/rust.git/src/liballoc/boxed.rs:682
#36 std::sys_common::thread::start_thread (main=0x7fffd1388890 "pN\366\320\377\177\000") at libstd/sys_common/thread.rs:24
#37 0x00007ffff7638e76 in std::sys::unix::thread::Thread::new::thread_start (main=0x0) at libstd/sys/unix/thread.rs:90
#38 0x00007ffff0d8c61b in start_thread () from /lib64/libpthread.so.0
#39 0x00007ffff731198f in clone () from /lib64/libc.so.6
(gdb)
