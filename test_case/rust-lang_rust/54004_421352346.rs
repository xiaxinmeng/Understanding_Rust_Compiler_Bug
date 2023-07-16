plain
[00:54:06] failures:
[00:54:06] 
[00:54:06] ---- [ui] ui/run-pass/panic-runtime/lto-abort.rs stdout ----
[00:54:06] 
[00:54:06] error: test compilation failed although it shouldn't!
[00:54:06] status: signal: 11
[00:54:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/run-pass/panic-runtime/lto-abort.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/run-pass/panic-runtime/lto-abort/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "lto" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/run-pass/panic-runtime/lto-abort/auxiliary" "-A" "unused"
[00:54:06] ------------------------------------------
[00:54:06] 
[00:54:06] ------------------------------------------
[00:54:06] stderr:
---
[00:54:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:06] 
[00:54:06] ---- [ui] ui/run-pass/panic-runtime/lto-unwind.rs stdout ----
[00:54:06] 
[00:54:06] error: test compilation failed although it shouldn't!
[00:54:06] status: signal: 11
[00:54:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/run-pass/panic-runtime/lto-unwind.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/run-pass/panic-runtime/lto-unwind/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "lto" "-C" "panic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/run-pass/panic-runtime/lto-unwind/auxiliary" "-A" "unused"
[00:54:06] ------------------------------------------
[00:54:06] 
[00:54:06] ------------------------------------------
[00:54:06] stderr:
---
[00:54:06] thread '[ui] ui/run-pass/panic-runtime/lto-unwind.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:54:06] 
[00:54:06] ---- [ui] ui/run-pass/sepcomp/sepcomp-lib-lto.rs stdout ----
[00:54:06] 
[00:54:06] error: test compilation failed although it shouldn't!
[00:54:06] status: signal: 11
[00:54:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/run-pass/sepcomp/sepcomp-lib-lto.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/run-pass/sepcomp/sepcomp-lib-lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "lto" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/run-pass/sepcomp/sepcomp-lib-lto/auxiliary" "-A" "unused"
[00:54:06] ------------------------------------------
[00:54:06] 
[00:54:06] ------------------------------------------
[00:54:06] stderr:
---
[00:54:06] 
[00:54:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:54:06] 
[00:54:06] 
[00:54:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:06] 
[00:54:06] 
[00:54:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:54:06] Build completed unsuccessfully in 0:50:23
---
travis_time:end:0116804f:start=1536930350159173391,finish=1536930350163973408,duration=4800017
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014da160
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.18433.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc...(no debugging symbols found)...done.
[New LWP 18554]
[New LWP 18433]
[New LWP 18441]
[New LWP 18442]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007ff4547ee436 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007ff4547ee436 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007ff4547ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007ff4547ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007ff4547ed725 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007ff4547ee378 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007ff4547ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007ff4547ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007ff4547ed725 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007ff4547ee378 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007ff4547ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007ff4547ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007ff4547c2d8d in llvm::DwarfCompileUnit::getOrCreateGlobalVariableDIE(llvm::DIGlobalVariable const*, llvm::ArrayRef<llvm::DwarfCompileUnit::GlobalExpr>) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007ff4547dad26 in llvm::DwarfDebug::beginModule() ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007ff45479fb4a in llvm::AsmPrinter::doInitialization(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007ff455337076 in llvm::FPPassManager::doInitialization(llvm::Module&) [clone .localalias.433] ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007ff45534117e in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007ff4538044cf in LLVMRustWriteOutputFile ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007ff4536c5c3a in rustc_codegen_llvm::back::write::write_output_file::h443b4f98d39d42ab ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007ff453692ec5 in rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::ha0711f63cbdb57e2 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#19 0x00007ff45368a108 in rustc::util::common::time_ext::h2cd9f9a7d1f47d76 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#20 0x00007ff4536c818a in rustc_codegen_llvm::back::write::codegen::h18e1d9c53f14953c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#21 0x00007ff4536ce5bd in rustc_codegen_llvm::back::write::execute_work_item::h41f9a6486c24240c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#22 0x00007ff4536a060d in std::sys_common::backtrace::__rust_begin_short_backtrace::h7184896e247b5bda ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#23 0x00007ff4536b75c6 in std::panicking::try::do_call::h1cc85788cb6cbfb3 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#24 0x00007ff45e4f637a in __rust_maybe_catch_panic ()
    at libpanic_unwind/lib.rs:102
#25 0x00007ff4536a8e41 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8361c485ea8bcee8 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#26 0x00007ff45e4e76cb in call_once<(),()> () at liballoc/boxed.rs:656
#27 std::sys_common::thread::start_thread::he8a1beb3bb672935 ()
    at libstd/sys_common/thread.rs:24
#28 0x00007ff45e4bb1c6 in std::sys::unix::thread::Thread::new::thread_start::h62b77adfe7ec52c1 () at libstd/sys/unix/thread.rs:90
#29 0x00007ff4586b16ba in ?? ()
#30 0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.18437.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc...(no debugging symbols found)...done.
[New LWP 18555]
[New LWP 18437]
[New LWP 18477]
[New LWP 18478]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007f51a79ee436 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007f51a79ee436 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007f51a79ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007f51a79ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007f51a79ed725 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007f51a79ee378 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007f51a79ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007f51a79ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007f51a79ed725 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007f51a79ee378 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007f51a79ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007f51a79ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007f51a79c2d8d in llvm::DwarfCompileUnit::getOrCreateGlobalVariableDIE(llvm::DIGlobalVariable const*, llvm::ArrayRef<llvm::DwarfCompileUnit::GlobalExpr>) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007f51a79dad26 in llvm::DwarfDebug::beginModule() ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007f51a799fb4a in llvm::AsmPrinter::doInitialization(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007f51a8537076 in llvm::FPPassManager::doInitialization(llvm::Module&) [clone .localalias.433] ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007f51a854117e in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007f51a6a044cf in LLVMRustWriteOutputFile ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007f51a68c5c3a in rustc_codegen_llvm::back::write::write_output_file::h443b4f98d39d42ab ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007f51a6892ec5 in rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::ha0711f63cbdb57e2 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#19 0x00007f51a688a108 in rustc::util::common::time_ext::h2cd9f9a7d1f47d76 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#20 0x00007f51a68c818a in rustc_codegen_llvm::back::write::codegen::h18e1d9c53f14953c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#21 0x00007f51a68ce5bd in rustc_codegen_llvm::back::write::execute_work_item::h41f9a6486c24240c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#22 0x00007f51a68a060d in std::sys_common::backtrace::__rust_begin_short_backtrace::h7184896e247b5bda ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#23 0x00007f51a68b75c6 in std::panicking::try::do_call::h1cc85788cb6cbfb3 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#24 0x00007f51b184437a in __rust_maybe_catch_panic ()
    at libpanic_unwind/lib.rs:102
#25 0x00007f51a68a8e41 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8361c485ea8bcee8 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#26 0x00007f51b18356cb in call_once<(),()> () at liballoc/boxed.rs:656
#27 std::sys_common::thread::start_thread::he8a1beb3bb672935 ()
    at libstd/sys_common/thread.rs:24
#28 0x00007f51b18091c6 in std::sys::unix::thread::Thread::new::thread_start::h62b77adfe7ec52c1 () at libstd/sys/unix/thread.rs:90
#29 0x00007f51ab9ff6ba in ?? ()
#30 0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.21982.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc...(no debugging symbols found)...done.
[New LWP 22090]
[New LWP 21982]
[New LWP 22024]
[New LWP 22025]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fa3a63ee436 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007fa3a63ee436 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fa3a63ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fa3a63ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fa3a63ed725 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fa3a63ee378 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fa3a63ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fa3a63ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fa3a63ed725 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fa3a63ee378 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fa3a63ecbb5 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fa3a63ecc34 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fa3a63c2d8d in llvm::DwarfCompileUnit::getOrCreateGlobalVariableDIE(llvm::DIGlobalVariable const*, llvm::ArrayRef<llvm::DwarfCompileUnit::GlobalExpr>) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fa3a63dad26 in llvm::DwarfDebug::beginModule() ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fa3a639fb4a in llvm::AsmPrinter::doInitialization(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fa3a6f37076 in llvm::FPPassManager::doInitialization(llvm::Module&) [clone .localalias.433] ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fa3a6f4117e in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fa3a54044cf in LLVMRustWriteOutputFile ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007fa3a52c5c3a in rustc_codegen_llvm::back::write::write_output_file::h443b4f98d39d42ab ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007fa3a5292ec5 in rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::ha0711f63cbdb57e2 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#19 0x00007fa3a528a108 in rustc::util::common::time_ext::h2cd9f9a7d1f47d76 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#20 0x00007fa3a52c818a in rustc_codegen_llvm::back::write::codegen::h18e1d9c53f14953c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#21 0x00007fa3a52ce5bd in rustc_codegen_llvm::back::write::execute_work_item::h41f9a6486c24240c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#22 0x00007fa3a52a060d in std::sys_common::backtrace::__rust_begin_short_backtrace::h7184896e247b5bda ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#23 0x00007fa3a52b75c6 in std::panicking::try::do_call::h1cc85788cb6cbfb3 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#24 0x00007fa3b010c37a in __rust_maybe_catch_panic ()
    at libpanic_unwind/lib.rs:102
#25 0x00007fa3a52a8e41 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8361c485ea8bcee8 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#26 0x00007fa3b00fd6cb in call_once<(),()> () at liballoc/boxed.rs:656
#27 std::sys_common::thread::start_thread::he8a1beb3bb672935 ()
    at libstd/sys_common/thread.rs:24
#28 0x00007fa3b00d11c6 in std::sys::unix::thread::Thread::new::thread_start::h62b77adfe7ec52c1 () at libstd/sys/unix/thread.rs:90
#29 0x00007fa3aa2c76ba in ?? ()
#30 0x0000000000000000 in ?? ()
travis_time:end:014da160:start=1536930350168277002,finish=1536930353423880788,duration=3255603786
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16757352
travis_time:start:16757352
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:043076e1
$ dmesg | grep -i kill
