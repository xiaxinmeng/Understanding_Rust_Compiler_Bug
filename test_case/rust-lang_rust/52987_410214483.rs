plain
[00:19:16]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:20:06] error: Could not compile `rustc_codegen_llvm`.
[00:20:06] 
[00:20:06] Caused by:
[00:20:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="jemalloc"' --cfg 'feature="rustc_target"' -C metadata=b42aa0d683b3ffda -C extra-filename=-b42aa0d683b3ffda --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-277ca49f42ab2e53.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-f28e2ea1414f1406.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-94b7c8f2f6f79e1b.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ba31465b048d39f4.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-00919937f377f0bc.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-18d8fee9014249a2.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-e4f798202172c031.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-dd0a9429b0aacf96.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-62a891846402192d.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-cf4f2299cca987aa.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-072369dae17b1893.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-780c150b6c3acb38.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-4f182245385237b5.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-5ed58e76c7dace45.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-1cc3798595d9708a.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-2e835ba951928190.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1ed4d2103c0d7730.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-899ce576c6b4bcbf.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-45767bac74e332c7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-71c6a17c4b962f9c/out -L native=/usr/lib/llvm-5.0/lib` (signal: 11, SIGSEGV: invalid memory reference)
[00:20:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:20:06] expected success, got: exit code: 101
[00:20:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1118:9
[00:20:06] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:20:06] travis_time:end:stage0-rustc_codegen_llvm:start=1533290596417434447,finish=1533290648532261785,duration=52114827338

[00:20:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:06] Build completed unsuccessfully in 0:16:19
[00:20:06] Makefile:28: recipe for target 'all' failed
[00:20:06] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02127ce0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2639a2e3:start=1533290649154168475,finish=1533290649161122591,duration=6954116
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ea78ad0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.8343.!checkout!obj!build!x86_64-unknown-linux-gnu!stage0!bin!rustc
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc...(no debugging symbols found)...done.
[New LWP 8375]
[New LWP 8372]
[New LWP 8373]
[New LWP 8343]
[New LWP 8344]
[New LWP 8374]
[New LWP 8345]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name rust'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fabd3b98182 in std::_Function_handler<bool (llvm::GlobalValue const&), llvm::thinLTOInternalizeModule(llvm::Module&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&)::$_2>::_M_invoke(std::_Any_data const&, llvm::GlobalValue const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007fabd3b98182 in std::_Function_handler<bool (llvm::GlobalValue const&), llvm::thinLTOInternalizeModule(llvm::Module&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&)::$_2>::_M_invoke(std::_Any_data const&, llvm::GlobalValue const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fabd3bb4b7e in llvm::InternalizePass::maybeInternalize(llvm::GlobalValue&, std::set<llvm::Comdat const*, std::less<llvm::Comdat const*>, std::allocator<llvm::Comdat const*> > const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fabd3bb4fe6 in llvm::InternalizePass::internalizeModule(llvm::Module&, llvm::CallGraph*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fabd3b94199 in llvm::thinLTOInternalizeModule(llvm::Module&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fabd3452558 in LLVMRustPrepareThinLTOInternalize ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fabd3356d2b in rustc_codegen_llvm::back::lto::LtoModuleCodegen::optimize::h509ac79a256a6094 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64----Type <return> to continue, or q <return> to quit---
