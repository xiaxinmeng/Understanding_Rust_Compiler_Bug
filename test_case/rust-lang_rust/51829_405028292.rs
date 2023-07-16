plain
[01:11:56] rustc exited with signal: 11
[01:11:56] error: Could not compile `rustc_codegen_llvm`.
[01:11:56] 
[01:11:56] Caused by:
[01:11:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=e13747a0333e79cd -C extra-filename=-e13747a0333e79cd --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5fdb96b18119744c.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-0fa0588f1c4412f5.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-895acd162c7c6552.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-62c67455b3319651.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-cd1507cdc2ba255a.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-b06ca4d7515cc070.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-f4c6ca12c55cd938.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-1513440731e90940.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-a064f5f4e13b8c1d.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-5c04b6300494bc09.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-236e74b289f15265.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-bd66201c87c6792c.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-37faab1b31eecbba.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-4de5b04ad8537345.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-8100b926b94dbf60.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-68d160adb2df29c3.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-5d9d07b86cb623bf.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-db6c274d71aca6a2.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-09130fa2a2fe1101.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5fdc9ff03f0e244b.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-54cc3d2ef16bda3d.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-54cc3d2ef16bda3d.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-175297c8bd84f82b.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3338f4d906a62ee8.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-701aa691529fbea1.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4266b9558e7a7bdc/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-94e3e427d24c98ad/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-8318ff5744d63dd8/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib -L native=/usr/lib/gcc/x86_64-linux-gnu/5` (exit code: 254)
[01:11:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[01:11:56] expected success, got: exit code: 101
[01:11:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[01:11:56] travis_fold:start:stage1-rustc_codegen_llvm
travis_time:start:stage1-rustc_codegen_llvm
travis_fold:end:stage1-rustc_codegen_llvm

---
travis_time:end:23d347af:start=1531579841127693240,finish=1531579841134529863,duration=6836623
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05829060
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.22316.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc...(no debugging symbols found)...done.
[New LWP 22324]
[New LWP 22323]
[New LWP 22321]
[New LWP 22316]
[New LWP 22317]
[New LWP 22325]
[New LWP 22318]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --crate-name rust'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007f9fc5fe6a11 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitOne(llvm::CallGraphNode*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007f9fc5fe6a11 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitOne(llvm::CallGraphNode*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007f9fc6f295b1 in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007f9fc7289f0c in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007f9fc720c829 in LLVMRunPassManager ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007f9fc5681fe7 in rustc_codegen_llvm::back::write::execute_work_item::hfd3cb98239864c6b ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007f9fc56343a3 in std::sys_common::backtrace::__rust_begin_short_backtrace::h8c37409cc6efa4b9 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007f9fc5658096 in std::panicking::try::do_call::h3fbe882ce88a0178 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007f9fcfc0942a in __rust_maybe_catch_panic ()
    at libpanic_unwind/lib.rs:106
#8  0x00007f9fc5641ad1 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hcd99e38bedf3313c ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007f9fcfbc13bb in call_once<(),()> ()
    at /checkout/src/liballoc/boxed.rs:650
#10 std::sys_common::thread::start_thread::hf91e8d5f6a6b3cf0 ()
    at libstd/sys_common/thread.rs:24
#11 0x00007f9fcfbc6bf6 in std::sys::unix::thread::Thread::new::thread_start::h3a956f4a27a51528 () at libstd/sys/unix/thread.rs:90
#12 0x00007f9fc9d196ba in ?? ()
#13 0x0000000000000000 in ?? ()
travis_time:end:05829060:start=1531579841141196222,finish=1531579842185010652,duration=1043814430
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06325780
travis_time:start:06325780
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11cc131a
$ dmesg | grep -i kill
