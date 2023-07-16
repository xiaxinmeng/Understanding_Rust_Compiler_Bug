
$ rm build/x86_64-unknown-linux-gnu/stage* -rf && time ./x.py build src/rustc
extracting /home/r/src/rust/rustc.2/build/cache/2018-09-11/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
extracting /home/r/src/rust/rustc.2/build/cache/2018-09-11/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
extracting /home/r/src/rust/rustc.2/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
    Finished dev [unoptimized] target(s) in 0.25s                                                                                                                                                                  
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling core v0.0.0 (file:///home/r/src/rust/rustc.2/src/libcore)
   Compiling unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libunwind)
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling compiler_builtins v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.33
   Compiling alloc_jemalloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///home/r/src/rust/rustc.2/src/libstd)
   Compiling rustc_asan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_asan)
   Compiling rustc_lsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lsan)
   Compiling rustc_tsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_tsan)
   Compiling rustc_msan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_msan)
   Compiling libc v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc)
   Compiling alloc_system v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_system)
   Compiling panic_abort v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_abort)
   Compiling panic_unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_unwind)
    Finished release [optimized] target(s) in 27.43s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling getopts v0.2.17                                                                                                                                                                                       
   Compiling term v0.0.0 (file:///home/r/src/rust/rustc.2/src/libterm)
   Compiling test v0.0.0 (file:///home/r/src/rust/rustc.2/src/libtest)
    Finished release [optimized] target(s) in 5.64s
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling version_check v0.1.4                                                                                                                                                                                  
   Compiling cfg-if v0.1.5
   Compiling nodrop v0.1.12
   Compiling scopeguard v0.3.3
   Compiling void v1.0.2
   Compiling memoffset v0.2.1
   Compiling rand_core v0.2.1
   Compiling stable_deref_trait v1.1.0
   Compiling libc v0.2.43
   Compiling rustc-rayon-core v0.1.1
   Compiling unicode-width v0.1.5
   Compiling either v1.5.0
   Compiling byteorder v1.2.3
   Compiling bitflags v1.0.4
   Compiling cc v1.0.25
   Compiling rustc_target v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_target)
   Compiling scoped-tls v0.1.2
   Compiling syntax v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax)
   Compiling termcolor v1.0.2
   Compiling lazy_static v0.2.11
   Compiling rustc-demangle v0.1.9
   Compiling remove_dir_all v0.5.1
   Compiling datafrog v0.1.0
   Compiling rustc v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc)
   Compiling graphviz v0.0.0 (file:///home/r/src/rust/rustc.2/src/libgraphviz)
   Compiling fmt_macros v0.0.0 (file:///home/r/src/rust/rustc.2/src/libfmt_macros)
   Compiling rustc_incremental v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_incremental)
   Compiling rustc_fs_util v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_fs_util)
   Compiling rustc_metadata v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_metadata)
   Compiling rustc-serialize v0.3.24
   Compiling quick-error v1.2.2
   Compiling rustc_platform_intrinsics v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_platform_intrinsics)
   Compiling rustc_driver v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_driver)
   Compiling crossbeam-utils v0.2.2
   Compiling log v0.4.4
   Compiling arrayvec v0.4.7
   Compiling unreachable v1.0.0
   Compiling owning_ref v0.3.3
   Compiling chalk-macros v0.1.0
   Compiling rustc-hash v1.0.1
   Compiling lazy_static v1.1.0
   Compiling rand v0.4.3
   Compiling num_cpus v1.8.0
   Compiling rand v0.5.5
   Compiling atty v0.2.11
   Compiling humantime v1.1.1
   Compiling smallvec v0.6.5
   Compiling lock_api v0.1.3
   Compiling ena v0.9.3
   Compiling rustc_cratesio_shim v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_cratesio_shim)
   Compiling jobserver v0.1.11
   Compiling polonius-engine v0.5.0
   Compiling chalk-engine v0.7.0
   Compiling serialize v0.0.0 (file:///home/r/src/rust/rustc.2/src/libserialize)
   Compiling env_logger v0.5.12
   Compiling backtrace-sys v0.1.24
   Compiling miniz-sys v0.1.10
   Compiling rustc_apfloat v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_apfloat)
   Compiling parking_lot_core v0.2.14
   Compiling parking_lot_core v0.3.0
   Compiling tempfile v3.0.3
   Compiling rls-span v0.4.0
   Compiling crossbeam-epoch v0.3.1
   Compiling log_settings v0.1.2
   Compiling parking_lot v0.6.4
   Compiling rls-data v0.18.1
   Compiling crossbeam-deque v0.2.0
   Compiling backtrace v0.3.9
   Compiling flate2 v1.0.2
   Compiling rustc-rayon v0.1.1
   Compiling rustc_data_structures v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_data_structures)
   Compiling arena v0.0.0 (file:///home/r/src/rust/rustc.2/src/libarena)
   Compiling syntax_pos v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax_pos)
   Compiling rustc_errors v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_errors)
   Compiling proc_macro v0.0.0 (file:///home/r/src/rust/rustc.2/src/libproc_macro)
   Compiling syntax_ext v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax_ext)
   Compiling rustc_metadata_utils v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_metadata_utils)
   Compiling rustc_mir v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_mir)
   Compiling rustc_typeck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_typeck)
   Compiling rustc_traits v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_traits)
   Compiling rustc_allocator v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_allocator)
   Compiling rustc_plugin v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_plugin)
   Compiling rustc_resolve v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_resolve)
   Compiling rustc_privacy v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_privacy)
   Compiling rustc_codegen_utils v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_utils)
   Compiling rustc_passes v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_passes)
   Compiling rustc_borrowck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_borrowck)
   Compiling rustc_lint v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lint)
   Compiling rustc_save_analysis v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_save_analysis)
   Compiling rustc-main v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc)
    Finished release [optimized] target(s) in 7m 15s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling rustc_codegen_llvm v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_llvm)
   Compiling libc v0.2.43
   Compiling rustc-demangle v0.1.9
   Compiling memmap v0.6.2
   Compiling num_cpus v1.8.0
   Compiling rustc_llvm v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_llvm)
warning: In file included from /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/TargetSubtargetInfo.h:22,
warning:                  from ../rustllvm/PassWrapper.cpp:29:
warning: /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/SchedulerRegistry.h: In constructor ‘llvm::RegisterScheduler::RegisterScheduler(const char*, const char*, llvm::RegisterScheduler::FunctionPassCtor)’:
warning: /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/SchedulerRegistry.h:40:52: warning: cast between incompatible function types from ‘llvm::RegisterScheduler::FunctionPassCtor’ {aka ‘llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)’} to ‘llvm::MachinePassCtor’ {aka ‘void* (*)()’} [-Wcast-function-type]
warning:    : MachinePassRegistryNode(N, D, (MachinePassCtor)C)
warning:                                                     ^
    Finished release [optimized] target(s) in 50.69s
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling core v0.0.0 (file:///home/r/src/rust/rustc.2/src/libcore)
   Compiling unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libunwind)
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling compiler_builtins v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.33
   Compiling alloc_jemalloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///home/r/src/rust/rustc.2/src/libstd)
   Compiling rustc_tsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_tsan)
   Compiling rustc_lsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lsan)
   Compiling rustc_msan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_msan)
   Compiling rustc_asan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_asan)
   Compiling libc v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc)
   Compiling alloc_system v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_system)
   Compiling panic_abort v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_abort)
   Compiling panic_unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_unwind)
    Finished release [optimized] target(s) in 39.07s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (file:///home/r/src/rust/rustc.2/src/libterm)                                                                                                                                             
   Compiling getopts v0.2.17
   Compiling test v0.0.0 (file:///home/r/src/rust/rustc.2/src/libtest)
    Finished release [optimized] target(s) in 7.96s
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling version_check v0.1.4                                                                                                                                                                                  
   Compiling nodrop v0.1.12
   Compiling cfg-if v0.1.5
   Compiling scopeguard v0.3.3
   Compiling memoffset v0.2.1
   Compiling void v1.0.2
   Compiling libc v0.2.43
   Compiling rustc-rayon-core v0.1.1
   Compiling stable_deref_trait v1.1.0
   Compiling rand_core v0.2.1
   Compiling byteorder v1.2.3
   Compiling either v1.5.0
   Compiling bitflags v1.0.4
   Compiling unicode-width v0.1.5
   Compiling cc v1.0.25
   Compiling scoped-tls v0.1.2
   Compiling rustc_target v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_target)
   Compiling termcolor v1.0.2
   Compiling syntax v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax)
   Compiling lazy_static v0.2.11
   Compiling remove_dir_all v0.5.1
   Compiling datafrog v0.1.0
   Compiling rustc-demangle v0.1.9
   Compiling rustc v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc)
   Compiling rustc_incremental v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_incremental)
   Compiling graphviz v0.0.0 (file:///home/r/src/rust/rustc.2/src/libgraphviz)
   Compiling rustc_fs_util v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_fs_util)
   Compiling fmt_macros v0.0.0 (file:///home/r/src/rust/rustc.2/src/libfmt_macros)
   Compiling rustc_metadata v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_metadata)
   Compiling rustc-serialize v0.3.24
   Compiling quick-error v1.2.2
   Compiling rustc_platform_intrinsics v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_platform_intrinsics)
   Compiling rustc_driver v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_driver)
   Compiling crossbeam-utils v0.2.2
   Compiling log v0.4.4
   Compiling arrayvec v0.4.7
   Compiling unreachable v1.0.0
   Compiling owning_ref v0.3.3
   Compiling rustc-hash v1.0.1
   Compiling chalk-macros v0.1.0
   Compiling rand v0.4.3
   Compiling rand v0.5.5
   Compiling num_cpus v1.8.0
   Compiling atty v0.2.11
   Compiling lazy_static v1.1.0
   Compiling humantime v1.1.1
   Compiling smallvec v0.6.5
   Compiling lock_api v0.1.3
   Compiling rustc_cratesio_shim v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_cratesio_shim)
   Compiling ena v0.9.3
   Compiling polonius-engine v0.5.0
   Compiling jobserver v0.1.11
   Compiling chalk-engine v0.7.0
   Compiling miniz-sys v0.1.10
   Compiling backtrace-sys v0.1.24
   Compiling rustc_apfloat v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_apfloat)
   Compiling env_logger v0.5.12
   Compiling parking_lot_core v0.2.14
   Compiling serialize v0.0.0 (file:///home/r/src/rust/rustc.2/src/libserialize)
   Compiling parking_lot_core v0.3.0
   Compiling tempfile v3.0.3
   Compiling crossbeam-epoch v0.3.1
   Compiling log_settings v0.1.2
   Compiling rls-span v0.4.0
   Compiling rls-data v0.18.1
   Compiling crossbeam-deque v0.2.0
   Compiling backtrace v0.3.9
   Compiling flate2 v1.0.2
   Compiling parking_lot v0.6.4
   Compiling rustc-rayon v0.1.1
   Compiling rustc_data_structures v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_data_structures)
   Compiling arena v0.0.0 (file:///home/r/src/rust/rustc.2/src/libarena)
   Compiling syntax_pos v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax_pos)
   Compiling rustc_errors v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_errors)
   Compiling proc_macro v0.0.0 (file:///home/r/src/rust/rustc.2/src/libproc_macro)
   Compiling syntax_ext v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax_ext)
   Compiling rustc_metadata_utils v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_metadata_utils)
   Compiling rustc_mir v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_mir)
   Compiling rustc_typeck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_typeck)
   Compiling rustc_allocator v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_allocator)
   Compiling rustc_traits v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_traits)
   Compiling rustc_resolve v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_resolve)
   Compiling rustc_plugin v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_plugin)
   Compiling rustc_privacy v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_privacy)
   Compiling rustc_codegen_utils v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_utils)
   Compiling rustc_borrowck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_borrowck)
   Compiling rustc_passes v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_passes)
   Compiling rustc_lint v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lint)
   Compiling rustc_save_analysis v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_save_analysis)
   Compiling rustc-main v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc)
    Finished release [optimized] target(s) in 10m 29s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling rustc_codegen_llvm v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_llvm)
   Compiling libc v0.2.43
   Compiling rustc-demangle v0.1.9
   Compiling num_cpus v1.8.0
   Compiling memmap v0.6.2
   Compiling rustc_llvm v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_llvm)
warning: In file included from /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/TargetSubtargetInfo.h:22,
warning:                  from ../rustllvm/PassWrapper.cpp:29:
warning: /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/SchedulerRegistry.h: In constructor ‘llvm::RegisterScheduler::RegisterScheduler(const char*, const char*, llvm::RegisterScheduler::FunctionPassCtor)’:
warning: /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/SchedulerRegistry.h:40:52: warning: cast between incompatible function types from ‘llvm::RegisterScheduler::FunctionPassCtor’ {aka ‘llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)’} to ‘llvm::MachinePassCtor’ {aka ‘void* (*)()’} [-Wcast-function-type]
warning:    : MachinePassRegistryNode(N, D, (MachinePassCtor)C)
warning:                                                     ^
    Finished release [optimized] target(s) in 1m 07s
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Uplifting stage1 test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:21:09

real	21m9,076s
user	114m16,748s
sys	1m2,455s
