plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1a8d7141
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:02:33]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:04:01] error: Could not compile `rustc`.
[01:04:01] 
[01:04:01] Caused by:
[01:04:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc src/librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=605a404c62e81837 -C extra-filename=-605a404c62e81837 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-272327a42837066f.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-60b6e510c3368f81.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-eb2f598266b9e1a1.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-f14cd72e64c32363.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-b0742107ee400c71.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-feff8ee3de29a960.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-d507fc3d268c5b2e.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-b56fdb34339fe92b.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-88df31933dfa412e.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-144b77090f62ae88.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-435e3408c3b4502e.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-65a72ea59e738e8a.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-0bd6f849edb21f64.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-e71cd28deda9d301.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-fb456fadc477bba9.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-5867afaa2924a633.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b491e9743ed06c34.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-14cfdda528ec2778.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-d989f7ca48994da8.so --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-69d29c352de30a1c.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e16f6e1462058c55.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-73ee63dcdac8fa1a.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-db646f5624409a50.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-db646f5624409a50.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-5fc7a5028055b005.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-c9a459f3ce426d2a.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-08a266c223f41ff7.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-6cbd630620c86fee.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-2443a2c730b16aa1/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-b572ae085766f8b0/out` (signal: 11, SIGSEGV: invalid memory reference)
[01:04:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[01:04:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:04:01] Build completed unsuccessfully in 0:44:04
travis_time:end:156af4e0:start=1549557066471940765,finish=1549560908290090777,duration=3841818150012
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:309aa1f4:start=1549560909637700661,finish=1549560909645048379,duration=7347718
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c15d10d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.15165.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc
[New LWP 15167]
[New LWP 15205]
[New LWP 15165]
[New LWP 15206]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib64/libpthread.so.0.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --edition=2018 --'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fe29c5ba352 in llvm::PointerType::get(llvm::Type*, unsigned int) () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
[Current thread is 1 (LWP 15167)]
#0  0x00007fe29c5ba352 in llvm::PointerType::get(llvm::Type*, unsigned int) () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#1  0x00007fe29c49cf63 in llvm::GetElementPtrInst::getGEPReturnType(llvm::Type*, llvm::Value*, llvm::ArrayRef<llvm::Value*>) () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#2  0x00007fe29c4c79e3 in llvm::GetElementPtrInst::Create(llvm::Type*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&, llvm::Instruction*) () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#3  0x00007fe29c4dfc3f in llvm::IRBuilder<llvm::ConstantFolder, llvm::IRBuilderDefaultInserter>::CreateConstInBoundsGEP2_32(llvm::Type*, llvm::Value*, unsigned int, unsigned int, llvm::Twine const&) () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#4  0x00007fe29c4dd092 in LLVMBuildStructGEP () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#5  0x00007fe29f1e4acf in _$LT$rustc_codegen_ssa..mir..place..PlaceRef$LT$$u27$tcx$C$$u20$V$GT$$GT$::project_field::_$u7b$$u7b$closure$u7d$$u7d$::h36422029cf3dc42f () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fe29f1e4541 in _$LT$rustc_codegen_ssa..mir..place..PlaceRef$LT$$u27$tcx$C$$u20$V$GT$$GT$::project_field::hd0d2310f737dfd67 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fe29f113a16 in rustc_codegen_ssa::mir::place::_$LT$impl$u20$rustc_codegen_ssa..mir..FunctionCx$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$Bx$GT$$GT$::codegen_place::hd628f2c84d7ab1c9 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fe29f115f3d in rustc_codegen_ssa::mir::rvalue::_$LT$impl$u20$rustc_codegen_ssa..mir..FunctionCx$LT$$u27$a$C$$u20$$u27$tcx$C$$u20$Bx$GT$$GT$::codegen_rvalue_operand::hfbe202af341cfee1 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fe29f10998b in rustc_codegen_ssa::mir::codegen_mir::h2aead98bb48fc1f2 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fe29f1d1e97 in rustc_codegen_ssa::base::codegen_instance::h82bdd351957dcc08 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fe29f21357c in rustc_codegen_ssa::mono_item::MonoItemExt::define::hf8343e45e0022853 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fe29f13df43 in rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::h48749a4680682c58 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fe29f232d32 in rustc::dep_graph::graph::DepGraph::with_task::hd01d335e232410d5 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fe29f14d798 in _$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_ssa..traits..backend..ExtraBackendMethods$GT$::compile_codegen_unit::hf1a159a7dbd4b7ef () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fe29f1cf077 in rustc_codegen_ssa::base::codegen_crate::ha4606c8ccf7c44ab () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fe29f14e421 in _$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_utils..codegen_backend..CodegenBackend$GT$::codegen_crate::he6ff28ebfe80c233 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007fe2a98d7894 in rustc::util::common::time::h4e4a90e2fc4d8fcf () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#18 0x00007fe2a99763e0 in rustc_driver::driver::phase_4_codegen::hdeabecd705c28b69 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#19 0x00007fe2a98e4142 in rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::hfdc9cb8f1e767dd9 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#20 0x00007fe2a98df925 in _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h2c792c4a8970aa6b () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#21 0x00007fe2a989d8d5 in rustc::ty::context::TyCtxt::create_and_enter::he5fec7a6b225057d () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#22 0x00007fe2a9973e08 in rustc_driver::driver::compile_input::hc668cc86fcc09067 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#23 0x00007fe2a99061f6 in _$LT$scoped_tls..ScopedKey$LT$T$GT$$GT$::set::h3ba20e19ce9c88a5 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#24 0x00007fe2a991fefb in rustc_driver::run_compiler::h0927532178f8a5e4 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#25 0x00007fe2a990710b in _$LT$scoped_tls..ScopedKey$LT$T$GT$$GT$::set::h7c5d3c1b603f4bdc () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#26 0x00007fe2a996be03 in syntax::with_globals::h237724a7b50df29e () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#27 0x00007fe2a958ca5a in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#28 0x00007fe2a9965411 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h7a5e95a5e2cd6d1a () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-11b92226e856e2ec.so
#29 0x00007fe2a958b81e in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$_$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h6d657de847213980 () at /rustc/9c905f840dc185bf8525d62f8059147eee74098e/src/liballoc/boxed.rs:744
#30 std::sys_common::thread::start_thread::h84e9771978345a15 () at src/libstd/sys_common/thread.rs:14
#31 std::sys::unix::thread::Thread::new::thread_start::hb94fd77b16c84faa () at src/libstd/sys/unix/thread.rs:81
#32 0x00007fe2a930083d in ?? ()
#33 0x0000000000000000 in ?? ()
travis_time:end:0c15d10d:start=1549560909649601133,finish=1549560911475575168,duration=1825974035
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ef063e8
travis_time:start:0ef063e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1362ac70
$ dmesg | grep -i kill
