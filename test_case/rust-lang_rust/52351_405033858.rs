plain
[00:48:17] rustc exited with signal: 11
[00:48:17] error: Could not compile `rustc_codegen_llvm`.
[00:48:17] 
[00:48:17] Caused by:
[00:48:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=3f1223e2fcaff230 -C extra-filename=-3f1223e2fcaff230 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps --target armv7-unknown-linux-gnueabihf -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libbitflags-7670e792a4a4f8b8.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libcc-f5a2d36e6aaaa74f.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libenv_logger-f7df1a53281490a3.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libflate2-f6e9c0bab4272e67.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libjobserver-f99b47de88e8e7b8.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/liblibc-64d31a5686dc219b.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/liblog-adb16a8516a8c2db.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libnum_cpus-46cca61fb72bf47b.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc-e79d67a57553fd73.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_demangle-32f633d2b18ce708.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_allocator-c0eb3438fdfa5265.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_apfloat-b3007f4d6a7b9b28.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_codegen_utils-7eec4dc6602a55c7.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_data_structures-0551f45d58884f6f.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_errors-f5cfbc6dc28bff64.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_incremental-0b25f75b8da658ad.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_llvm-329465d538fd71fa.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_mir-82d735c1d2f4a8fc.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_platform_intrinsics-f38228b53277bf2e.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/librustc_target-63dc6ee3afd0bdcb.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libserialize-ef9ad3fd510378f6.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libserialize-ef9ad3fd510378f6.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libsyntax-c194908b1383f48e.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libsyntax_pos-dab0d36b998491a4.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/deps/libtempfile-99bc25c311ab6ecf.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/build/miniz-sys-37efc03b158f9266/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/build/backtrace-sys-d960b2f342054351/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/armv7-unknown-linux-gnueabihf/release/build/rustc_llvm-21c5ff36ddc94e6e/out -L native=/checkout/obj/build/armv7-unknown-linux-gnueabihf/llvm/build/lib -L native=/x-tools/armv7-unknown-linux-gnueabihf/lib/gcc/armv7-unknown-linux-gnueabihf/4.9.3/../../../../armv7-unknown-linux-gnueabihf/lib` (exit code: 254)
[00:48:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "armv7-unknown-linux-gnueabihf" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:48:17] expected success, got: exit code: 101
[00:48:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:48:17] travis_fold:start:stage1-rustc_codegen_llvm
travis_time:start:stage1-rustc_codegen_llvm
travis_fold:end:stage1-rustc_codegen_llvm

---
travis_time:end:00ac7f6d:start=1531585240291128542,finish=1531585240297661983,duration=6533441
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10218806
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.20048.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc...(no debugging symbols found)...done.
[New LWP 20056]
[New LWP 20048]
[New LWP 20053]
[New LWP 20055]
[New LWP 20049]
[New LWP 20057]
[New LWP 20050]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --crate-name rust'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fa14675aa11 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitOne(llvm::CallGraphNode*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007fa14675aa11 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitOne(llvm::CallGraphNode*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fa14769d5b1 in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fa1479fdf0c in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fa147980829 in LLVMRunPassManager ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fa145df5fe7 in rustc_codegen_llvm::back::write::execute_work_item::hfd3cb98239864c6b ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fa145da83a3 in std::sys_common::backtrace::__rust_begin_short_backtrace::h8c37409cc6efa4b9 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fa145dcc096 in std::panicking::try::do_call::h3fbe882ce88a0178 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fa1503c242a in __rust_maybe_catch_panic ()
    at libpanic_unwind/lib.rs:106
#8  0x00007fa145db5ad1 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hcd99e38bedf3313c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fa15037a3bb in call_once<(),()> ()
    at /checkout/src/liballoc/boxed.rs:650
#10 std::sys_common::thread::start_thread::hf91e8d5f6a6b3cf0 ()
    at libstd/sys_common/thread.rs:24
#11 0x00007fa15037fbf6 in std::sys::unix::thread::Thread::new::thread_start::h3a956f4a27a51528 () at libstd/sys/unix/thread.rs:90
#12 0x00007fa14a48d6ba in ?? ()
#13 0x0000000000000000 in ?? ()
travis_time:end:10218806:start=1531585240302704374,finish=1531585241370238052,duration=1067533678
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2c1a4fcc
travis_time:start:2c1a4fcc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1894c0d8
$ dmesg | grep -i kill
