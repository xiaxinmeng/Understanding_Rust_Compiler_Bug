plain
[00:59:30] rustc exited with signal: 11
[00:59:30] error: Could not compile `rustc_codegen_llvm`.
[00:59:30] 
[00:59:30] Caused by:
[00:59:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=5403c8224f4410c7 -C extra-filename=-5403c8224f4410c7 --out-dir /checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps --target i686-unknown-linux-gnu -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libbitflags-e4d5933f06f67e73.rlib --extern cc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libcc-72200e1a85a2c72b.rlib --extern env_logger=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libenv_logger-03e0088cb60a6228.rlib --extern flate2=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libflate2-b6856453122081c8.rlib --extern jobserver=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libjobserver-ba90d18067f06a15.rlib --extern libc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/liblibc-f9843a8199807cf8.rlib --extern log=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/liblog-5bcae59244124eba.rlib --extern num_cpus=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libnum_cpus-b96ed89c24b6554b.rlib --extern rustc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc-ccefab8e8ec63a8d.so --extern rustc_demangle=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_demangle-9ff7768a7003d792.rlib --extern rustc_allocator=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_allocator-b3eaa22a811222e6.so --extern rustc_apfloat=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_apfloat-760f350e8bb9eaab.rlib --extern rustc_codegen_utils=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_codegen_utils-832aeedb2aa74604.so --extern rustc_data_structures=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_data_structures-809d00a6ec91e864.so --extern rustc_errors=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_errors-9f5db6760eee664f.so --extern rustc_incremental=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_incremental-365f738274464e73.so --extern rustc_llvm=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_llvm-ea8b0677cf20f7b7.rlib --extern rustc_mir=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_mir-a9eb524e1b9b567f.so --extern rustc_platform_intrinsics=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-9518f7701e433e23.so --extern rustc_target=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/librustc_target-0eeba5c96b4d8151.so --extern serialize=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libserialize-d6b2572c6655dc27.so --extern serialize=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libserialize-d6b2572c6655dc27.rlib --extern syntax=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libsyntax-a961e2045676d2c0.so --extern syntax_pos=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libsyntax_pos-8a01075255c9e1ee.so --extern tempfile=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/deps/libtempfile-3398117a2c3fc96d.rlib -L native=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/build/miniz-sys-450c553de1fe2030/out -L native=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/build/backtrace-sys-6e4f00109ff8ade4/out -L native=/checkout/obj/build/i686-unknown-linux-gnu/stage1-rustc/i686-unknown-linux-gnu/release/build/rustc_llvm-5fe19504400ac87c/out -L native=/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/lib` (exit code: 254)
[00:59:30] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:59:30] expected success, got: exit code: 101
[00:59:30] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:59:30] travis_fold:start:stage1-rustc_codegen_llvm
travis_time:start:stage1-rustc_codegen_llvm
travis_fold:end:stage1-rustc_codegen_llvm


[00:59:30] travis_time:end:stage1-rustc_codegen_llvm:start=1531575222033896146,finish=1531575261648564304,duration=39614668158

[00:59:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:59:30] Build completed unsuccessfully in 0:55:17
[00:59:30] Makefile:28: recipe for target 'all' failed
[00:59:30] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c7bec30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0984682e:start=1531575262463298095,finish=1531575262477659195,duration=14361100
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1766f9aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.29989.!checkout!obj!build!i686-unknown-linux-gnu!stage1!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/i686-unknown-linux-gnu/stage1/bin/rustc...(no debugging symbols found)...done.
[New LWP 29997]
[New LWP 29994]
[New LWP 29996]
[New LWP 29989]
[New LWP 29990]
[New LWP 29991]
warning: Could not load shared library symbols for 8 libraries, e.g. /lib32/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/i686-unknown-linux-gnu/stage1/bin/rustc --crate-name rustc_'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0xf060dfd4 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitOne(llvm::CallGraphNode*) ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0xf060dfd4 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitOne(llvm::CallGraphNode*) ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0xf18923f5 in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0xf1ce2992 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0xf1ce2c16 in llvm::legacy::PassManager::run(llvm::Module&) ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0xf1c11707 in LLVMRunPassManager ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0xefabe765 in rustc::util::common::time_ext::ha86a1cfd2f804883 ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0xef9f4fc3 in rustc_codegen_llvm::back::write::execute_work_item::hd3287bfffbba4523 ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0xef99d4a5 in std::sys_common::backtrace::__rust_begin_short_backtrace::h896d831a2965dfe2 ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0xef9a1b99 in std::panicking::try::do_call::h3f54d2d7238ab12f ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0xf7421aa8 in __rust_maybe_catch_panic ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/bin/../lib/libstd-3b53412ca929654e.so
#10 0xef9a1958 in std::panicking::try::h3084146c3124b6e8 ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0xef9b035f in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h1e377b64dc36c8ca ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0xf7411de4 in std::sys_common::thread::start_thread::hdb772644013fe568 ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/bin/../lib/libstd-3b53412ca929654e.so
#13 0xf740919c in std::sys::unix::thread::Thread::new::thread_start::hc33cc835ee2f8c70 ()
   from ./checkout/obj/build/i686-unknown-linux-gnu/stage1/bin/../lib/libstd-3b53412ca929654e.so
#14 0xf42a026a in ?? ()
#15 0xd71007d0 in ?? ()
#16 0xf72af41e in ?? ()
travis_time:end:1766f9aa:start=1531575262487271734,finish=1531575263991037674,duration=1503765940
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11c3d8f0
travis_time:start:11c3d8f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f329400
$ dmesg | grep -i kill
