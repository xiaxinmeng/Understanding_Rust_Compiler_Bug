
$ rm build/x86_64-unknown-linux-gnu/stage* -rf && time ./x.py build src/rustc
extracting /home/r/src/rust/rustc.2/build/cache/2018-09-11/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
extracting /home/r/src/rust/rustc.2/build/cache/2018-09-11/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
extracting /home/r/src/rust/rustc.2/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
   Compiling bootstrap v0.0.0 (file:///home/r/src/rust/rustc.2/src/bootstrap)                                                                                                                                      
    Finished dev [unoptimized] target(s) in 17.46s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling core v0.0.0 (file:///home/r/src/rust/rustc.2/src/libcore)
   Compiling unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libunwind)
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling compiler_builtins v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.33
   Compiling alloc_jemalloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///home/r/src/rust/rustc.2/src/libstd)
   Compiling rustc_lsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lsan)
   Compiling rustc_tsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_tsan)
   Compiling rustc_msan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_msan)
   Compiling rustc_asan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_asan)
   Compiling libc v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc)
   Compiling alloc_system v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_system)
   Compiling panic_abort v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_abort)
   Compiling panic_unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_unwind)
    Finished release [optimized] target(s) in 26.28s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (file:///home/r/src/rust/rustc.2/src/libterm)                                                                                                                                             
   Compiling getopts v0.2.17
   Compiling test v0.0.0 (file:///home/r/src/rust/rustc.2/src/libtest)
    Finished release [optimized] target(s) in 5.22s
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling version_check v0.1.4                                                                                                                                                                                  
   Compiling cfg-if v0.1.5
   Compiling nodrop v0.1.12
   Compiling memoffset v0.2.1
   Compiling void v1.0.2
   Compiling scopeguard v0.3.3
   Compiling rustc-rayon-core v0.1.1
   Compiling stable_deref_trait v1.1.0
   Compiling libc v0.2.43
   Compiling rand_core v0.2.1
   Compiling byteorder v1.2.3
   Compiling unicode-width v0.1.5
   Compiling bitflags v1.0.4
   Compiling either v1.5.0
   Compiling scoped-tls v0.1.2
   Compiling cc v1.0.25
   Compiling rustc_target v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_target)
   Compiling syntax v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax)
   Compiling termcolor v1.0.2
   Compiling lazy_static v0.2.11
   Compiling rustc-demangle v0.1.9
   Compiling rustc v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc)
   Compiling datafrog v0.1.0
   Compiling remove_dir_all v0.5.1
   Compiling fmt_macros v0.0.0 (file:///home/r/src/rust/rustc.2/src/libfmt_macros)
   Compiling rustc_fs_util v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_fs_util)
   Compiling graphviz v0.0.0 (file:///home/r/src/rust/rustc.2/src/libgraphviz)
   Compiling rustc_incremental v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_incremental)
   Compiling rustc-serialize v0.3.24
   Compiling rustc_metadata v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_metadata)
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
   Compiling rand v0.4.3
   Compiling num_cpus v1.8.0
   Compiling rand v0.5.5
   Compiling atty v0.2.11
   Compiling lazy_static v1.1.0
   Compiling humantime v1.1.1
   Compiling smallvec v0.6.5
   Compiling lock_api v0.1.3
   Compiling rustc_cratesio_shim v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_cratesio_shim)
   Compiling ena v0.9.3
   Compiling jobserver v0.1.11
   Compiling polonius-engine v0.5.0
   Compiling chalk-engine v0.7.0
   Compiling rustc_apfloat v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_apfloat)
   Compiling backtrace-sys v0.1.24
   Compiling miniz-sys v0.1.10
   Compiling serialize v0.0.0 (file:///home/r/src/rust/rustc.2/src/libserialize)
   Compiling parking_lot_core v0.2.14
   Compiling env_logger v0.5.12
   Compiling parking_lot_core v0.3.0
   Compiling tempfile v3.0.3
   Compiling crossbeam-epoch v0.3.1
   Compiling log_settings v0.1.2
   Compiling parking_lot v0.6.4
   Compiling rls-span v0.4.0
   Compiling crossbeam-deque v0.2.0
   Compiling backtrace v0.3.9
   Compiling rls-data v0.18.1
   Compiling flate2 v1.0.2
   Compiling rustc-rayon v0.1.1
   Compiling rustc_data_structures v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_data_structures)
   Compiling arena v0.0.0 (file:///home/r/src/rust/rustc.2/src/libarena)
   Compiling syntax_pos v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax_pos)
   Compiling rustc_errors v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_errors)
   Compiling proc_macro v0.0.0 (file:///home/r/src/rust/rustc.2/src/libproc_macro)
   Compiling syntax_ext v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax_ext)
   Compiling rustc_mir v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_mir)
   Compiling rustc_metadata_utils v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_metadata_utils)
   Compiling rustc_typeck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_typeck)
   Compiling rustc_allocator v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_allocator)
   Compiling rustc_traits v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_traits)
   Compiling rustc_plugin v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_plugin)
   Compiling rustc_resolve v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_resolve)
   Compiling rustc_privacy v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_privacy)
   Compiling rustc_codegen_utils v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_utils)
   Compiling rustc_lint v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lint)
   Compiling rustc_borrowck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_borrowck)
   Compiling rustc_passes v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_passes)
   Compiling rustc_save_analysis v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_save_analysis)
   Compiling rustc-main v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc)
    Finished release [optimized] target(s) in 6m 38s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/home/r/src/rust/rustc.2/src/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DPYTHON_EXECUTABLE=/usr/bin/python" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
-- Native target architecture is X86
-- Threads enabled.
-- Doxygen disabled.
-- Go bindings disabled.
-- OCaml bindings disabled, need ctypes >=0.4.
-- Found Python module pygments
-- Found Python module pygments.lexers.c_cpp
-- Found Python module yaml
-- LLVM host triple: x86_64-unknown-linux-gnu
-- LLVM default target triple: x86_64-unknown-linux-gnu
-- Building with -fPIC
-- Constructing LLVMBuild project information
-- Linker detection: GNU ld
-- Targeting X86
-- Targeting ARM
-- Targeting AArch64
-- Targeting Mips
-- Targeting PowerPC
-- Targeting SystemZ
-- Targeting MSP430
-- Targeting Sparc
-- Targeting NVPTX
-- Targeting Hexagon
-- Targeting WebAssembly
-- Targeting RISCV
-- Configuring done
-- Generating done
-- Build files have been written to: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[8/73] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCISelLowering.cpp.o
In file included from /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/TargetSubtargetInfo.h:22,
                 from /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/MachineRegisterInfo.h:32,
                 from /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/LiveRegUnits.h:19,
                 from /home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/TargetInstrInfo.h:21,
                 from /home/r/src/rust/rustc.2/src/llvm/lib/Target/PowerPC/PPCInstrInfo.h:19,
                 from /home/r/src/rust/rustc.2/src/llvm/lib/Target/PowerPC/PPCISelLowering.h:19,
                 from /home/r/src/rust/rustc.2/src/llvm/lib/Target/PowerPC/PPCISelLowering.cpp:14:
/home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/SchedulerRegistry.h: In constructor ‘llvm::RegisterScheduler::RegisterScheduler(const char*, const char*, llvm::RegisterScheduler::FunctionPassCtor)’:
/home/r/src/rust/rustc.2/src/llvm/include/llvm/CodeGen/SchedulerRegistry.h:40:52: warning: cast between incompatible function types from ‘llvm::RegisterScheduler::FunctionPassCtor’ {aka ‘llvm::ScheduleDAGSDNodes* (*)(llvm::SelectionDAGISel*, llvm::CodeGenOpt::Level)’} to ‘llvm::MachinePassCtor’ {aka ‘void* (*)()’} [-Wcast-function-type]
   : MachinePassRegistryNode(N, D, (MachinePassCtor)C)
                                                    ^
[72/73] Install the project...
-- Install configuration: "Release"
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize/DIPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize/SymbolizableModule.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize/Symbolize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/GlobalTypeTableBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugLinesSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/RecordSerialization.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/StringsAndChecksums.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CodeViewSymbols.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CodeView.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CodeViewError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolRecordMapping.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugChecksumsSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugUnknownSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/EnumTables.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolVisitorDelegate.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeSymbolEmitter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CodeViewTypes.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CVRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/Formatters.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeDeserializer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/FunctionId.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/GUID.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugFrameDataSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CodeViewRegisters.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugSubsectionRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolSerializer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/LazyRandomTypeCollection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugSymbolsSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugSymbolRVASubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeRecordMapping.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CVSymbolVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/ContinuationRecordBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/Line.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CVTypeVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/CodeViewRecordIO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolVisitorCallbacks.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugCrossImpSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugInlineeLinesSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/RecordName.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/MergingTypeTableBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeIndexDiscovery.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SimpleTypeSerializer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolDeserializer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeHashing.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugSubsectionVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/AppendingTypeTableBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugStringTableSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeStreamMerger.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeDumpVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolDumpDelegate.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeCollection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeIndex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolDumper.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeVisitorCallbacks.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/SymbolVisitorCallbackPipeline.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeVisitorCallbackPipeline.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/DebugCrossExSubsection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/CodeView/TypeTableCollection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFRelocMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugAbbrev.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFExpression.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugRnglists.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFCompileUnit.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFAttribute.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugLoc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDataExtractor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFContext.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFListTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFFormValue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugArangeSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugInfoEntry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugRangeList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFTypeUnit.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFUnitIndex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFUnit.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFObject.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFGdbIndex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugLine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugAddr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFAcceleratorTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugAranges.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFAbbreviationDeclaration.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugPubTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugMacro.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDie.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFAddressRange.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugFrame.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFVerifier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFSection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolThunk.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolFuncDebugStart.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/GenericError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolUnknown.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumSectionContribs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASectionContrib.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASession.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASupport.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASourceFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIARawSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIADataStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumLineNumbers.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIATable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumSymbols.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIALineNumber.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumDebugStreams.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumTables.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumSourceFiles.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAInjectedSource.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumInjectedSources.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeArray.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBLineNumber.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeBaseClass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolUsingNamespace.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolPublicSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeEnum.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymDumper.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolAnnotation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolCompilandEnv.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolCompilandDetails.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeFriend.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBEnumChildren.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBSectionContrib.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeDimension.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolExe.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeUDT.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBDataStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/UDTLayout.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBSourceFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeTypedef.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeCustom.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBSession.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBContext.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolBlock.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeVTableShape.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolFunc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDB.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolCompiland.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBExtras.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolLabel.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeManaged.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeBuiltin.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypePointer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeFunctionSig.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBInjectedSource.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBRawSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolCustom.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/ConcreteSymbolEnumerator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/DbiStreamBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/SymbolStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeEnumModules.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/PDBFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/ISectionContribVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/DbiModuleList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/EnumTables.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/DbiModuleDescriptorBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/TpiHashing.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeCompilandSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/InfoStreamBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/TpiStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/RawTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/Formatters.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeExeSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeEnumSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/InfoStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/TpiStreamBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/DbiStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/PublicsStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeRawSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/Hash.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/HashTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/GSIStreamBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NamedStreamMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/RawConstants.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeSession.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/DbiModuleDescriptor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeBuiltinSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/PDBFileBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/PDBStringTableBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/RawError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/PDBStringTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeEnumTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolData.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolFuncDebugEnd.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeFunctionArg.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeVTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DIContext.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/IMSFFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFCommon.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MappedBlockStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AArch64TargetParser.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CommandLine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormattedStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TaskQueue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MemAlloc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DataTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Chrono.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FileSystem.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/circular_raw_ostream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Program.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/NativeFormatting.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStreamReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Error.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Options.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CBindingWrapping.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ItaniumManglingCanonicalizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GraphWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/UnicodeCharRanges.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MutexGuard.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MathExtras.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TargetSelect.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SystemUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DOTGraphTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DebugCounter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ManagedStatic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GenericDomTreeConstruction.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Host.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LockFileManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStreamRef.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Capacity.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormatCommon.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CFGUpdate.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TargetParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStreamWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TrigramIndex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LineIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/JSON.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Process.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BranchProbability.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Memory.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ThreadLocal.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormatAdapters.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryItemStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MD5.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TargetOpcodes.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Valgrind.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Endian.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/PrettyStackTrace.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormatProviders.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Allocator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/X86DisassemblerDecoderCommon.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LICENSE.TXT
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/StringSaver.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/RandomNumberGenerator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/JamCRC.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TrailingObjects.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Atomic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TargetRegistry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FileCheck.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStreamArray.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/WindowsError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SwapByteOrder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ArrayRecycler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SpecialCaseList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/thread.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CrashRecoveryContext.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ARMWinEH.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/EndianStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GlobPattern.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ThreadPool.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SaveAndRestore.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GenericDomTree.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ARMTargetParser.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MachineValueType.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/X86TargetParser.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ConvertUTF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Compression.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/InitLLVM.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ErrorOr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Timer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/YAMLParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/raw_sha1_ostream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/OnDiskHashTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ARMAttributeParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ToolOutputFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FileUtilities.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/PluginLoader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScaledNumber.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Watchdog.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AMDGPUMetadata.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/VersionTuple.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/type_traits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CachePruning.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DJB.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/RWMutex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Errc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Unicode.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CheckedArithmetic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AlignOf.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Locale.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/RecyclingAllocator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TypeName.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Registry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/raw_os_ostream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Solaris
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Solaris/sys
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Solaris/sys/regset.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStreamError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Mutex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DataExtractor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AtomicOrdering.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/UniqueLock.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Threading.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/PointerLikeTypeTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MipsABIFlags.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BlockFrequency.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Debug.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/WithColor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TarWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ReverseIteration.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CodeGenCoverage.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SHA1.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ErrorHandling.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Parallel.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Regex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Compiler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Win64EH.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/KnownBits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryByteStream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Errno.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SMLoc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SourceMgr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScopedPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/xxhash.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SmallVectorMemoryBuffer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ARMEHABI.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LowLevelTypeImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Signals.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/StringPool.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Recycler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Format.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DynamicLibrary.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LEB128.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormatVariadicDetails.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CodeGen.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/YAMLTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormatVariadic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MemoryBuffer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/COM.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/raw_ostream.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Casting.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ARMBuildAttributes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Path.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FileOutputBuffer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AMDHSAKernelDescriptor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Printable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IRReader
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IRReader/IRReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/Option.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/OptTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/Arg.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/OptParser.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/OptSpecifier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/ArgList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LineEditor
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LineEditor/LineEditor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/YAMLXRayRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Trace.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Graph.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/InstrumentationMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/XRayRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FileHeaderReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/DCE.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/ADCE.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/GuardWidening.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/AlignmentFromAssumptions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/CallSiteSplitting.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SROA.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LICM.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopSink.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopRotation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/MemCpyOptimizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SpeculateAroundPHIs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LowerGuardIntrinsic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/BDCE.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/PartiallyInlineLibCalls.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/Reassociate.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/InductiveRangeCheckElimination.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SpeculativeExecution.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopIdiomRecognize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LowerExpectIntrinsic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopDataPrefetch.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/InstSimplifyPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/GVNExpression.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/JumpThreading.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopPassManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SimplifyCFG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/EarlyCSE.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopAccessAnalysisPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/Sink.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/IndVarSimplify.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopStrengthReduce.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopUnrollAndJamPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopInstSimplify.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopUnrollPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopPredication.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/DivRemPairs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SimpleLoopUnswitch.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SCCP.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/MergedLoadStoreMotion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/Float2Int.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/RewriteStatepointsForGC.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LowerAtomic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/CorrelatedValuePropagation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/NewGVN.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/TailRecursionElimination.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/IVUsersPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/GVN.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/NaryReassociate.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopSimplifyCFG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopLoadElimination.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopDistribute.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/DeadStoreElimination.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/ConstantHoisting.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopDeletion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize/LoopVectorize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize/LoopVectorizationLegality.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize/SLPVectorizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Coroutines.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LowerInvoke.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/GlobalStatus.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/PredicateInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SanitizerStats.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ModuleUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ImportedFunctionsInliningStatistics.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/BreakCriticalEdges.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/CodeExtractor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LibCallsShrinkWrap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopRotationUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/UnifyFunctionExitNodes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SymbolRewriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopSimplify.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LowerMemIntrinsics.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdater.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Local.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterBulk.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/CtorUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SimplifyIndVar.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Mem2Reg.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LCSSA.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/CallPromotionUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/VNCoercion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/InstructionPrecedenceTracking.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/BuildLibCalls.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/BypassSlowDivision.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Cloning.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/FunctionImportUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ASanStackFrameLayout.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/UnrollLoop.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SimplifyLibCalls.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/AddDiscriminators.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/NameAnonGlobals.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/GuardUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopVersioning.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/EscapeEnumerator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/PromoteMemToReg.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/EntryExitInstrumenter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/FunctionComparator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SplitModule.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Evaluator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/BasicBlockUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ValueMapper.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/IntegerDivision.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/OrderedInstructions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/InstCombine
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/InstCombine/InstCombine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/InstCombine/InstCombineWorklist.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/ObjCARC.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/AggressiveInstCombine
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/AggressiveInstCombine/AggressiveInstCombine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/GCOVProfiler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/CGProfile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/PGOInstrumentation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/InstrProfiling.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/BoundsChecking.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/FunctionAttrs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/DeadArgumentElimination.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ArgumentPromotion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/CrossDSOCFI.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ThinLTOBitcodeWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ConstantMerge.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SyntheticCountsPropagation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/GlobalDCE.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/CalledValuePropagation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/PassManagerBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/StripDeadPrototypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/WholeProgramDevirt.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SCCP.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/AlwaysInliner.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/InferFunctionAttrs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/LowerTypeTests.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/PartialInlining.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SampleProfile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/FunctionImport.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/GlobalSplit.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ElimAvailExtern.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/Inliner.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/GlobalOpt.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ForceFunctionAttrs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/Internalize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/FuzzerCLI.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/Operations.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/IRMutator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/RandomIRBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/OpDescriptor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/Random.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/GCOV.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/InstrProfData.inc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/InstrProf.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/SampleProfReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/ProfileCommon.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/Coverage
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/Coverage/CoverageMappingReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/Coverage/CoverageMappingWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/InstrProfReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/InstrProfWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/SampleProf.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/SampleProfWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/LTOBackend.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/legacy
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/legacy/LTOModule.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/legacy/ThinLTOCodeGenerator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/legacy/UpdateCompilerUsed.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/legacy/LTOCodeGenerator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/LTO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/Config.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LTO/Caching.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/PostDominators.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CFGPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CFLAliasAnalysisUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AssumptionCache.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MustExecute.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/RegionInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DominanceFrontierImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CmpInstAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/PhiValues.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/SparsePropagation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ValueTracking.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/EHPersonalities.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CallGraph.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/RegionPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ValueLatticeUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Lint.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/OrderedBasicBlock.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/RegionInfoImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DemandedBits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ScalarEvolutionNormalization.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MemorySSAUpdater.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopInfoImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IntervalPartition.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Loads.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CallPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TargetLibraryInfo.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/BasicAliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyValueInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CFG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CaptureTracking.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IntervalIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TypeMetadataUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/BlockFrequencyInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TargetTransformInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/OptimizationRemarkEmitter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Trace.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IteratedDominanceFrontier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ScalarEvolutionAliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Interval.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/BranchProbabilityInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DominanceFrontier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ProfileSummaryInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MemoryDependenceAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Utils
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Utils/Local.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/PtrUseVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopUnrollAnalyzer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ScalarEvolutionExpander.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AliasSetTracker.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ConstantFolding.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DivergenceAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/InlineCost.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ScalarEvolutionExpressions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DependenceAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/InstructionSimplify.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TargetFolder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ModuleSummaryAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopAccessAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CallGraphSCCPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Passes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MemoryBuiltins.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IndirectCallSiteVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyBlockFrequencyInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CFLSteensAliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ObjCARCInstKind.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/RegionIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IndirectCallPromotionAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ObjCARCAliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DomPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/RegionPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ScalarEvolution.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CodeMetrics.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ObjCARCAnalysisUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AliasAnalysisEvaluator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CGSCCPassManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/VectorUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CFLAndersAliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TargetTransformInfoImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/PHITransAddr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopAnalysisManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyCallGraph.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/SyntheticCountsUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ScopedNoAliasAA.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/GlobalsModRef.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/BlockFrequencyInfoImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IVUsers.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ValueLattice.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MemorySSA.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyBranchProbabilityInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MemoryLocation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TargetLibraryInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DOTGraphTraitsPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TypeBasedAliasAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource/ResourceProcessor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource/ResourceScriptToken.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource/ResourceScriptTokenList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetInstrPredicate.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GenericOpcodes.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/CodeGenCWrappers.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetOptions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetSchedule.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/Target.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetIntrinsicInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetLoweringObjectFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GlobalISel
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GlobalISel/Target.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GlobalISel/SelectionDAGCompat.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GlobalISel/RegisterBank.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetMachine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetCallingConv.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetSelectionDAG.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetItinerary.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/InitializePasses.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/AArch64.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/Lanai.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/Sparc.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/i386.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/Hexagon.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/RISCV.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/ARC.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/AVR.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/SystemZ.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/ARM.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/BPF.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/AMDGPU.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC64.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/Mips.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/x86_64.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Dwarf.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Magic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPack.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/COFF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MachO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPackReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MachO.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPackWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Dwarf.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/WasmRelocs.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/DynamicTags.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPack.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Wasm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/AsmParser
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/AsmParser/Parser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/AsmParser/SlotMapping.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/MCAsmParserUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/MCAsmParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/MCAsmLexer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/MCTargetAsmParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/MCParsedAsmOperand.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/AsmLexer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/AsmCond.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCParser/MCAsmParserExtension.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSchedule.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCValue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInstrInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWinCOFFObjectWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCTargetOptionsCommandFlags.inc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmBackend.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInstrAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInstBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MachineLocation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInstPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCELFStreamer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAssembler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmInfoDarwin.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCObjectStreamer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCELFObjectWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCLabel.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCStreamer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDirectives.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCCodePadder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/SubtargetFeature.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/LaneBitmask.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmInfoWasm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSubtargetInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCCodeView.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSection.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCObjectWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSectionWasm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmInfoCOFF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCFragment.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmInfoELF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCRegisterInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbolELF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDwarf.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWinEH.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWin64EH.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWinCOFFStreamer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbolCOFF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCTargetOptions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInst.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmLayout.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/SectionKind.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbolWasm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCFixup.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCLinkerOptimizationHint.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCObjectFileInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbolMachO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCExpr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSectionELF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCMachObjectWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWasmStreamer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInstrItineraries.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCCodeEmitter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCFixupKindInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCContext.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCFixedLenDisassembler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSectionCOFF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDisassembler
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDisassembler/MCRelocationInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDisassembler/MCExternalSymbolizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDisassembler/MCDisassembler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCDisassembler/MCSymbolizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWasmObjectWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCInstrDesc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSectionMachO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/ConstantPools.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCAsmMacro.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/StringTableBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Passes
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Passes/PassPlugin.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Passes/PassBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassRegistry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/YAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/WasmYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLSymbols.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLTypeHashing.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLDebugSections.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/ObjectYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/ELFYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/COFFYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/MachOYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/DWARFEmitter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/DWARFYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ToolDrivers
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ToolDrivers/llvm-dlltool
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ToolDrivers/llvm-dlltool/DlltoolDriver.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ToolDrivers/llvm-lib
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ToolDrivers/llvm-lib/LibDriver.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MIRPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGNodes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/VirtRegMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetFrameLowering.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LowLevelType.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ValueTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineConstantPool.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/EdgeBundles.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineTraceMetrics.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MIRParser
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MIRParser/MIRParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineMemOperand.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveRangeEdit.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ISDOpcodes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PreISelIntrinsicLowering.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MIRYamlMapping.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveInterval.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CalcSpillWeights.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegisterClassInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScheduleHazardRecognizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/DAGCombine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineSSAUpdater.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetSchedule.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGISel.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CommandFlags.inc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGTargetInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GCMetadata.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGAddressAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CostTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineFunction.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineDominators.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveRegUnits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/AccelTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/DFAPacketizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/DIE.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/WinEHFuncInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MacroFusion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LivePhysRegs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ParallelCG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScheduleDAGMutation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TailDuplicator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LoopTraversal.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQPRAConstraint.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetInstrInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineBasicBlock.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineCombinerPattern.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GCs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SDNodeProperties.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ExecutionDomainFix.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveRegMatrix.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CallingConvLower.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegAllocRegistry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineInstrBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/WasmEHFuncInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LatencyPriorityQueue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/UnreachableBlockElim.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveStacks.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScheduleDFS.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegisterScavenging.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineInstr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/FaultMaps.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineLoopInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachinePassRegistry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/AsmPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineFrameInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SlotIndexes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineBlockFrequencyInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegAllocPBQP.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineJumpTableInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/Passes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineModuleInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineRegionInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineRegisterInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/IntrinsicLowering.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/DIEValue.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveIntervals.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveVariables.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SchedulerRegistry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/StackMaps.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineModuleInfoImpls.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/StackProtector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachORelocation.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GCStrategy.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/FunctionLoweringInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScheduleDAGInstrs.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegisterUsageInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GCMetadataPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineOperand.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScheduleDAG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ValueTypes.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineBranchProbabilityInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetLowering.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineOutliner.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineScheduler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/Analysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineInstrBundleIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/BasicTTIImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LazyMachineBlockFrequencyInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RuntimeLibcalls.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineDominanceFrontier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetLoweringObjectFileImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/InstructionSelectorImpl.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/CombinerInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/RegisterBank.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/GISelWorkList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/LegalizerHelper.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/RegBankSelect.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/Legalizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/CallLowering.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/InstructionSelector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/RegisterBankInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/LegalizationArtifactCombiner.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/ConstantFoldingMIRBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/InstructionSelect.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/Localizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/Combiner.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/LegalizerInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/MIPatternMatch.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/MachineIRBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/Types.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/IRTranslator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/CombinerHelper.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/Utils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineInstrBundle.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetOpcodes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ExpandReductions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PseudoSourceValue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/DwarfStringPoolEntry.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegisterPressure.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineFunctionPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/FastISel.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveIntervalUnion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ReachingDefAnalysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachinePostDominators.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetCallingConv.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/AtomicExpandUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQP
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQP/Solution.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQP/Graph.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQP/ReductionRules.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQP/CostAllocator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PBQP/Math.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LinkAllAsmWriterComponents.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScoreboardHazardRecognizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetRegisterInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LexicalScopes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetPassConfig.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LinkAllCodegenComponents.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineOptimizationRemarkEmitter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ResourcePriorityQueue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetSubtargetInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassSupport.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Pass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/MachOUniversal.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Error.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ELFTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/IRObjectFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ArchiveWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ELF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ObjectFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/COFF.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/MachO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ELFObjectFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/IRSymtab.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/RelocVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/COFFModuleDefinition.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Archive.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ModuleSymbolTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/CVDebugRecord.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/StackMapParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolSize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/WindowsResource.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolicFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Wasm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/COFFImportFile.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Binary.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/WasmTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Decompressor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/StringSwitch.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/IntervalMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/BreadthFirstIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PointerEmbeddedInt.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/DenseSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ilist_node.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Any.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Sequence.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/CachedHashString.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SetOperations.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SparseMultiSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PriorityWorklist.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/DenseMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PointerSumType.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Twine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/UniqueVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/BitVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/TinyPtrVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ImmutableList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/DAGDeltaAlgorithm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ilist_base.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SparseSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/iterator_range.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ScopedHashTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/edit_distance.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ilist.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/APInt.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/FoldingSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SmallString.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/simple_ilist.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SmallSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SmallBitVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/IntEqClasses.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/StringRef.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Statistic.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PointerIntPair.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/StringSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Optional.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/STLExtras.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SetVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SmallPtrSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/BitmaskEnum.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/DepthFirstIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/DenseMapInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ImmutableSet.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/GraphTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ilist_node_base.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PointerUnion.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/IndexedMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ScopeExit.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ImmutableMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/DeltaAlgorithm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/APSInt.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/EpochTracker.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Triple.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/FunctionExtras.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PostOrderIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ilist_iterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PriorityQueue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/IntrusiveRefCntPtr.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ArrayRef.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/PackedVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/VariadicFunction.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/StringExtras.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/AllocatorList.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/ilist_node_options.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/None.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/MapVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SCCIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/iterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/Hashing.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SparseBitVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/StringMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/SmallVector.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/APFloat.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ADT/EquivalenceClasses.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Linker
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Linker/IRMover.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Linker/Linker.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassAnalysisSupport.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsManifest
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsManifest/WindowsManifestMerger.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/RuntimeDyldChecker.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/RTDyldMemoryManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/SectionMemoryManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/MCJIT.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/OProfileWrapper.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/RuntimeDyld.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/GenericValue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/SymbolStringPool.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/Core.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcABISupport.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/IRTransformLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/ExecutionUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/RemoteObjectLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/RPCSerialization.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/NullResolver.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/LLJIT.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/ObjectTransformLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetServer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/IndirectionUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/CompileOnDemandLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/RTDyldObjectLinkingLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/Legacy.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/GlobalMappingLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/Layer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/LazyEmittingLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/RawByteChannel.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcError.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/IRCompileLayer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/RPCUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/LambdaResolver.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/CompileUtils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetRPCAPI.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/OrcMCJITReplacement.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Interpreter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/ExecutionEngine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/ObjectCache.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITEventListener.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LinkAllPasses.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/SupportHelpers.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Error.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/StringView.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/Utility.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/ItaniumDemangle.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/Compiler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/Demangle.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/BitcodeWriterPass.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/BitstreamWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/BitstreamReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/BitCodes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/LLVMBitCodes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/BitcodeReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Bitcode/BitcodeWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LinkAllIR.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/Error.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/SearchableTable.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/TableGenBackend.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/Record.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/StringMatcher.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/Main.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/StringToOffsetTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/SetTheory.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DiagnosticPrinter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsARM.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ModuleSlotTracker.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PassManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/OperandTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/AssemblyAnnotationWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LegacyPassManagers.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Dominators.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/InstIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Constants.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicInst.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/MDBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Intrinsics.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsBPF.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DerivedUser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/NoFolder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Function.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsMips.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IRPrintingPasses.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Module.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsNVVM.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PatternMatch.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/TrackingMDRef.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LegacyPassManager.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ModuleSummaryIndex.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Mangler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/CallingConv.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsAArch64.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/BasicBlock.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LegacyPassNameParser.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Instructions.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DataLayout.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/RuntimeLibcalls.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/CFG.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalObject.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Operator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DiagnosticHandler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugInfoMetadata.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Statepoint.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/AutoUpgrade.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalIndirectSymbol.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GetElementPtrTypeIterator.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/InstrTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/TypeFinder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/CallSite.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Use.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsSystemZ.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/SymbolTableListTraits.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalAlias.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Instruction.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/InlineAsm.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PredIteratorCache.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsWebAssembly.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/User.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsPowerPC.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/OptBisect.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Constant.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PassTimingInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Type.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/InstVisitor.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ValueHandle.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Value.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ModuleSummaryIndexYAML.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ConstantFolder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/SafepointIRVerifier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalVariable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ConstantRange.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/UseListOrder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Intrinsics.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/CFGDiff.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalValue.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugLoc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ProfileSummary.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Verifier.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DiagnosticInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DIBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsXCore.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ValueSymbolTable.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GVMaterializer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LLVMContext.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PassManagerInternal.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IRBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/ValueMap.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DerivedTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Argument.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsX86.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalIFunc.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugInfoFlags.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsHexagon.td
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Instruction.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/TypeBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DomTreeUpdater.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Value.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Comdat.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/DataTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Core.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Error.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/BitWriter.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Support.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/InstCombine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Vectorize.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Coroutines.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/IPO.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/PassManagerBuilder.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Scalar.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Utils.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/BitReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/DebugInfo.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/IRReader.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/lto.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/OrcBindings.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/ErrorHandling.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Target.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Analysis.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/ExecutionEngine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/TargetMachine.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/DisassemblerTypes.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Disassembler.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Types.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Linker.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/LinkTimeOptimizer.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Initialization.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Object.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Comdat.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/Disassemblers.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/llvm-config.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/Targets.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/abi-breaking.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/AsmParsers.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/AsmPrinters.def
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/VCSRevision.h
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicImpl.inc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.inc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicEnums.inc
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDemangle.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSupport.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMTableGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMCore.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMFuzzMutate.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMIRReader.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSelectionDAG.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMIRParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMGlobalISel.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBinaryFormat.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitReader.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitWriter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMTransformUtils.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstrumentation.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAggressiveInstCombine.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstCombine.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMScalarOpts.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMipo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMVectorize.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/LLVMHello.so
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMObjCARCOpts.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMCoroutines.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMLinker.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAnalysis.a
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMLTO.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMC.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMCParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMCDisassembler.a
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMObject.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMObjectYAML.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMOption.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoDWARF.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoMSF.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoCodeView.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoPDB.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSymbolize.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMExecutionEngine.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInterpreter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMCJIT.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMOrcJIT.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRuntimeDyld.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMTarget.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86CodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86AsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86Disassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86AsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86Desc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86Info.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86Utils.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMARMUtils.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64CodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64AsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64Disassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64AsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64Desc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64Info.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAArch64Utils.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMipsCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMipsAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMipsDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMipsAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMipsDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMipsInfo.a
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPowerPCCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPowerPCAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPowerPCDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPowerPCAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPowerPCDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPowerPCInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSystemZCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSystemZAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSystemZDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSystemZAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSystemZDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSystemZInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMSP430CodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMSP430AsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMSP430Desc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMSP430Info.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSparcCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSparcAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSparcDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSparcAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSparcDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSparcInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMNVPTXCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMNVPTXAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMNVPTXDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMNVPTXInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMHexagonCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMHexagonAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMHexagonDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMHexagonDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMHexagonInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWebAssemblyCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWebAssemblyAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWebAssemblyDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWebAssemblyAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWebAssemblyDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWebAssemblyInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRISCVCodeGen.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRISCVAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRISCVDisassembler.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRISCVAsmPrinter.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRISCVDesc.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRISCVInfo.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAsmParser.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMLineEditor.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMProfileData.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMCoverage.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMPasses.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDlltoolDriver.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMLibDriver.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMXRay.a
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMWindowsManifest.a
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLTO.so.8svn
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLTO.so
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/lto.h
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-ar
Creating llvm-ranlib
Creating llvm-lib
Creating llvm-dlltool
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-config
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-lto
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/bugpoint
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/BugpointPasses.so
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/dsymutil
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llc
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/lli
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-as
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-bcanalyzer
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-c-test
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cat
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cfi-verify
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cov
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cvtres
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cxxdump
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cxxfilt
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-diff
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dis
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dwarfdump
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dwp
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-exegesis
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-extract
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-link
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-lto2
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mc
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mca
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-modextract
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mt
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-nm
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-objcopy
Creating llvm-strip
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-objdump
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-opt-report
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-pdbutil
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-rc
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-readobj
Creating llvm-readelf
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-rtdyld
Creating libLLVM-8.0.0svn.so
Creating libLLVM.so
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVM-8svn.so
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-size
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-split
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-stress
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-strings
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-symbolizer
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-undname
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-xray
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/obj2yaml
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/opt
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/opt-diff.py
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/opt-stats.py
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/opt-viewer.py
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/optpmap.py
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/optrecord.py
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/style.css
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/sancov
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/sanstats
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/verify-uselistorder
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/bin/yaml2obj
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMExports.cmake
-- Installing: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMExports-release.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMConfig.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMConfigVersion.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVM-Config.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/.
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./CheckCompilerVersion.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./VersionFromVCS.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./LLVMProcessSources.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddLLVMDefinitions.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./HandleLLVMOptions.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./CheckAtomic.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./ChooseMSVCCRT.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./GenerateVersionFromCVS.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./CrossCompile.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./DetermineGCCCompatible.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./HandleLLVMStdlib.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddOCaml.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindLibpfm.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./LLVMInstallSymlink.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindSphinx.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./CheckLinkerFlag.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./TableGen.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindOCaml.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddSphinxTarget.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddLLVM.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./GetSVN.cmake
-- Up-to-date: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./LLVMExternalProjectUtils.cmake
cargo:root=/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/llvm
	finished in 28.979
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling libc v0.2.43
   Compiling rustc_codegen_llvm v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_llvm)
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
    Finished release [optimized] target(s) in 47.59s
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling core v0.0.0 (file:///home/r/src/rust/rustc.2/src/libcore)
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)
   Compiling unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libunwind)
   Compiling compiler_builtins v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.33
   Compiling alloc_jemalloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///home/r/src/rust/rustc.2/src/libstd)
   Compiling rustc_asan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_asan)
   Compiling rustc_lsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lsan)
   Compiling rustc_msan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_msan)
   Compiling rustc_tsan v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_tsan)
   Compiling libc v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc)
   Compiling alloc_system v0.0.0 (file:///home/r/src/rust/rustc.2/src/liballoc_system)
   Compiling panic_abort v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_abort)
   Compiling panic_unwind v0.0.0 (file:///home/r/src/rust/rustc.2/src/libpanic_unwind)
    Finished release [optimized] target(s) in 38.22s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling getopts v0.2.17                                                                                                                                                                                       
   Compiling term v0.0.0 (file:///home/r/src/rust/rustc.2/src/libterm)
   Compiling test v0.0.0 (file:///home/r/src/rust/rustc.2/src/libtest)
    Finished release [optimized] target(s) in 7.49s
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling version_check v0.1.4                                                                                                                                                                                  
   Compiling nodrop v0.1.12
   Compiling cfg-if v0.1.5
   Compiling memoffset v0.2.1
   Compiling void v1.0.2
   Compiling scopeguard v0.3.3
   Compiling stable_deref_trait v1.1.0
   Compiling rustc-rayon-core v0.1.1
   Compiling rand_core v0.2.1
   Compiling libc v0.2.43
   Compiling unicode-width v0.1.5
   Compiling byteorder v1.2.3
   Compiling either v1.5.0
   Compiling bitflags v1.0.4
   Compiling rustc_target v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_target)
   Compiling cc v1.0.25
   Compiling scoped-tls v0.1.2
   Compiling termcolor v1.0.2
   Compiling syntax v0.0.0 (file:///home/r/src/rust/rustc.2/src/libsyntax)
   Compiling lazy_static v0.2.11
   Compiling rustc v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc)
   Compiling rustc-demangle v0.1.9
   Compiling remove_dir_all v0.5.1
   Compiling datafrog v0.1.0
   Compiling rustc_incremental v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_incremental)
   Compiling graphviz v0.0.0 (file:///home/r/src/rust/rustc.2/src/libgraphviz)
   Compiling fmt_macros v0.0.0 (file:///home/r/src/rust/rustc.2/src/libfmt_macros)
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
   Compiling backtrace-sys v0.1.24
   Compiling miniz-sys v0.1.10
   Compiling env_logger v0.5.12
   Compiling tempfile v3.0.3
   Compiling rustc_apfloat v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_apfloat)
   Compiling parking_lot_core v0.3.0
   Compiling parking_lot_core v0.2.14
   Compiling serialize v0.0.0 (file:///home/r/src/rust/rustc.2/src/libserialize)
   Compiling rls-span v0.4.0
   Compiling crossbeam-epoch v0.3.1
   Compiling log_settings v0.1.2
   Compiling parking_lot v0.6.4
   Compiling backtrace v0.3.9
   Compiling rls-data v0.18.1
   Compiling crossbeam-deque v0.2.0
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
   Compiling rustc_lint v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_lint)
   Compiling rustc_borrowck v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_borrowck)
   Compiling rustc_save_analysis v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_save_analysis)
   Compiling rustc-main v0.0.0 (file:///home/r/src/rust/rustc.2/src/rustc)
    Finished release [optimized] target(s) in 9m 36s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling build_helper v0.1.0 (file:///home/r/src/rust/rustc.2/src/build_helper)                                                                                                                                
   Compiling cc v1.0.25
   Compiling libc v0.2.43
   Compiling rustc_codegen_llvm v0.0.0 (file:///home/r/src/rust/rustc.2/src/librustc_codegen_llvm)
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
    Finished release [optimized] target(s) in 1m 01s
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Uplifting stage1 test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:20:13

real	20m13,125s
user	108m10,488s
sys	1m0,044s
