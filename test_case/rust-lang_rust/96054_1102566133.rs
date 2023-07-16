
(cd /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9 ; /usr/bin/env CONFIG_SHELL="/bin/bash" PKG_CONFIG_PATH="/usr/lib/sparcv9/pkgconfig" CC="/usr/gcc/11/bin/gcc" CXX="/usr/gcc/11/bin/g++" PATH="/usr/bin/sparcv9:/builds/psumbera/RUST/solaris-userland/components/rust/rustc/epoll::/builds/psumbera/rustc-1.59.0/bin:/usr/gnu/bin:/usr/bin" CC_FOR_BUILD="/usr/gcc/11/bin/gcc -m64" CXX_FOR_BUILD="/usr/gcc/11/bin/g++ -m64" CPPFLAGS="-m64" "ac_cv_func_realloc_0_nonnull=yes" "NM=/usr/gnu/bin/nm" INTLTOOL_PERL="/usr/perl5/5.22/bin/perl" CFLAGS="-m64 -O3 -mno-app-regs" CXXFLAGS="-m64 -O3 -mno-app-regs" LDFLAGS="" /bin/bash \
	/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/configure --default-linker=gcc --enable-local-rust)
configure: processing command line
configure: 
configure: rust.default-linker  := gcc
configure: build.rustc          := /builds/psumbera/rustc-1.59.0/bin/rustc
configure: build.cargo          := /builds/psumbera/rustc-1.59.0/bin/cargo
configure: build.configure-args := ['--default-linker=gcc', '--enable-local-rust' ...
configure: 
configure: writing `config.toml` in current directory
configure: 
configure: run `python /builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/x.py --help`
configure: 
/usr/bin/touch /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/.configured
cd /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build; mkdir -p .cargo; echo "[source.crates-io]" > .cargo/config; echo "replace-with = \"vendored-sources\"" >> .cargo/config; echo "[source.vendored-sources]" >> .cargo/config; echo "directory = \"/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/vendor\"" >> .cargo/config;
(cd /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9 ; /usr/bin/env LD_OPTIONS="-M /usr/lib/ld/map.noexbss -M /usr/lib/ld/map.pagealign  -zignore -zstrip-class=comment" LD_EXEC_OPTIONS=" -zsx=aslr=enable -zsx=nxstack=enable -zsx=nxheap=enable  " LD_PIE_OPTIONS="-zsx=aslr=enable -zsx=nxstack=enable -zsx=nxheap=enable  "  PKG_CONFIG_PATH="/usr/lib/sparcv9/pkgconfig" "ac_cv_func_realloc_0_nonnull=yes" INTLTOOL_PERL="/usr/perl5/5.22/bin/perl" CARGO_HOME=/builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/.cargo \
	/usr/gnu/bin/make -j 32 -l 256  )
make[1]: Entering directory '/builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9'
Building rustbuild
   Compiling proc-macro2 v1.0.30
   Compiling unicode-xid v0.2.2
   Compiling memchr v2.4.1
   Compiling syn v1.0.80
   Compiling serde_derive v1.0.125
   Compiling cfg-if v1.0.0
   Compiling serde v1.0.125
   Compiling lazy_static v1.4.0
   Compiling log v0.4.14
   Compiling libc v0.2.116
   Compiling crossbeam-utils v0.8.6
   Compiling regex-automata v0.1.10
   Compiling ryu v1.0.5
   Compiling regex-syntax v0.6.25
   Compiling once_cell v1.7.2
   Compiling fnv v1.0.7
   Compiling serde_json v1.0.59
   Compiling same-file v1.0.6
   Compiling bootstrap v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/src/bootstrap)
   Compiling itoa v0.4.6
   Compiling unicode-width v0.1.8
   Compiling cc v1.0.69
   Compiling build_helper v0.1.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/src/build_helper)
   Compiling opener v0.5.0
   Compiling getopts v0.2.21
   Compiling walkdir v2.3.1
   Compiling thread_local v1.1.4
   Compiling aho-corasick v0.7.18
   Compiling bstr v0.2.13
   Compiling cmake v0.1.44
   Compiling filetime v0.2.14
   Compiling num_cpus v1.13.1
   Compiling quote v1.0.7
   Compiling regex v1.5.4
   Compiling globset v0.4.5
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 2m 24s
Building stage0 std artifacts (sparcv9-sun-solaris -> sparcv9-sun-solaris)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/core)
   Compiling libc v0.2.116
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
   Compiling rustc-std-workspace-alloc v1.99.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/panic_unwind)
   Compiling panic_abort v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/panic_abort)
   Compiling gimli v0.25.0
   Compiling std_detect v0.1.5 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling object v0.26.2
   Compiling hashbrown v0.12.0
   Compiling addr2line v0.16.0
   Compiling rustc-std-workspace-std v1.99.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/rustc-std-workspace-std)
   Compiling proc_macro v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/library/test)
    Finished release [optimized] target(s) in 3m 31s
Copying stage0 std from stage0 (sparcv9-sun-solaris -> sparcv9-sun-solaris / sparcv9-sun-solaris)
Building LLVM for sparcv9-sun-solaris
running: "cmake" "/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/src/llvm-project/llvm" "-DCMAKE_SYSTEM_NAME=SunOS" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=256" "-DLLVM_TARGET_ARCH=sparcv9" "-DLLVM_DEFAULT_TARGET_TRIPLE=sparcv9-sun-solaris" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-dev" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=gcc" "-DCMAKE_CXX_COMPILER=g++" "-DCMAKE_ASM_COMPILER=gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_SHARED_LINKER_FLAGS=" "-DCMAKE_MODULE_LINKER_FLAGS=" "-DCMAKE_EXE_LINKER_FLAGS=" "-DCMAKE_INSTALL_PREFIX=/builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC" "-DCMAKE_BUILD_TYPE=Release"
-- The C compiler identification is GNU 11.2.0
-- The CXX compiler identification is GNU 10.3.0
-- The ASM compiler identification is GNU
-- Found assembler: /builds/psumbera/RUST/solaris-userland/components/rust/rustc/epoll/gcc
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: /builds/psumbera/RUST/solaris-userland/components/rust/rustc/epoll/gcc - skipped
-- Detecting C compile features
-- Detecting C compile features - done
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Check for working CXX compiler: /usr/bin/g++ - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Looking for dlfcn.h
-- Looking for dlfcn.h - found
-- Looking for errno.h
-- Looking for errno.h - found
-- Looking for fcntl.h
-- Looking for fcntl.h - found
-- Looking for link.h
-- Looking for link.h - found
-- Looking for malloc/malloc.h
-- Looking for malloc/malloc.h - not found
-- Looking for pthread.h
-- Looking for pthread.h - found
-- Looking for signal.h
-- Looking for signal.h - found
-- Looking for sys/ioctl.h
-- Looking for sys/ioctl.h - found
-- Looking for sys/mman.h
-- Looking for sys/mman.h - found
-- Looking for sys/param.h
-- Looking for sys/param.h - found
-- Looking for sys/resource.h
-- Looking for sys/resource.h - found
-- Looking for sys/stat.h
-- Looking for sys/stat.h - found
-- Looking for sys/time.h
-- Looking for sys/time.h - found
-- Looking for sys/types.h
-- Looking for sys/types.h - found
-- Looking for sysexits.h
-- Looking for sysexits.h - found
-- Looking for termios.h
-- Looking for termios.h - found
-- Looking for unistd.h
-- Looking for unistd.h - found
-- Looking for valgrind/valgrind.h
-- Looking for valgrind/valgrind.h - not found
-- Looking for fenv.h
-- Looking for fenv.h - found
-- Looking for FE_ALL_EXCEPT
-- Looking for FE_ALL_EXCEPT - found
-- Looking for FE_INEXACT
-- Looking for FE_INEXACT - found
-- Looking for mach/mach.h
-- Looking for mach/mach.h - not found
-- Looking for histedit.h
-- Looking for histedit.h - found
-- Looking for CrashReporterClient.h
-- Looking for CrashReporterClient.h - not found
-- Looking for pthread_create in pthread
-- Looking for pthread_create in pthread - found
-- Looking for pthread_getspecific in pthread
-- Looking for pthread_getspecific in pthread - found
-- Looking for pthread_rwlock_init in pthread
-- Looking for pthread_rwlock_init in pthread - found
-- Looking for pthread_mutex_lock in pthread
-- Looking for pthread_mutex_lock in pthread - found
-- Looking for dlopen in dl
-- Looking for dlopen in dl - found
-- Looking for clock_gettime in rt
-- Looking for clock_gettime in rt - found
-- Looking for pfm_initialize in pfm
-- Looking for pfm_initialize in pfm - not found
-- Looking for pthread.h
-- Looking for pthread.h - found
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD - Success
-- Found Threads: TRUE  
-- Found ZLIB: /usr/lib/64/libz.so (found version "1.2.11") 
-- Looking for compress2
-- Looking for compress2 - found
-- Looking for xar_open in xar
-- Looking for xar_open in xar - not found
-- Looking for arc4random
-- Looking for arc4random - found
-- Looking for backtrace
-- Looking for backtrace - found
-- backtrace facility detected in default set of libraries
-- Found Backtrace: /usr/include  
-- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW
-- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Failed
-- Looking for __register_frame
-- Looking for __register_frame - found
-- Looking for __deregister_frame
-- Looking for __deregister_frame - found
-- Looking for __unw_add_dynamic_fde
-- Looking for __unw_add_dynamic_fde - not found
-- Looking for _Unwind_Backtrace
-- Looking for _Unwind_Backtrace - found
-- Looking for getpagesize
-- Looking for getpagesize - found
-- Looking for sysconf
-- Looking for sysconf - found
-- Looking for getrusage
-- Looking for getrusage - found
-- Looking for setrlimit
-- Looking for setrlimit - found
-- Looking for isatty
-- Looking for isatty - found
-- Looking for futimens
-- Looking for futimens - found
-- Looking for futimes
-- Looking for futimes - not found
-- Looking for sigaltstack
-- Looking for sigaltstack - found
-- Looking for lseek64
-- Looking for lseek64 - found
-- Looking for mallctl
-- Looking for mallctl - not found
-- Looking for mallinfo
-- Looking for mallinfo - not found
-- Looking for mallinfo2
-- Looking for mallinfo2 - not found
-- Looking for malloc_zone_statistics
-- Looking for malloc_zone_statistics - not found
-- Looking for getrlimit
-- Looking for getrlimit - found
-- Looking for posix_spawn
-- Looking for posix_spawn - found
-- Looking for pread
-- Looking for pread - found
-- Looking for sbrk
-- Looking for sbrk - found
-- Looking for strerror
-- Looking for strerror - found
-- Looking for strerror_r
-- Looking for strerror_r - found
-- Looking for strerror_s
-- Looking for strerror_s - not found
-- Looking for setenv
-- Looking for setenv - found
-- Looking for dlopen
-- Looking for dlopen - found
-- Looking for dladdr
-- Looking for dladdr - found
-- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC
-- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC - Failed
-- Performing Test HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC
-- Performing Test HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC - Success
-- Looking for __GLIBC__
-- Looking for __GLIBC__ - not found
-- Looking for pthread_getname_np
-- Looking for pthread_getname_np - found
-- Looking for pthread_setname_np
-- Looking for pthread_setname_np - found
-- Looking for proc_pid_rusage
-- Looking for proc_pid_rusage - not found
-- Performing Test HAVE_STD_IS_TRIVIALLY_COPYABLE
-- Performing Test HAVE_STD_IS_TRIVIALLY_COPYABLE - Success
-- Performing Test HAVE_CXX_ATOMICS_WITHOUT_LIB
-- Performing Test HAVE_CXX_ATOMICS_WITHOUT_LIB - Success
-- Performing Test HAVE_CXX_ATOMICS64_WITHOUT_LIB
-- Performing Test HAVE_CXX_ATOMICS64_WITHOUT_LIB - Success
-- Performing Test LLVM_HAS_ATOMICS
-- Performing Test LLVM_HAS_ATOMICS - Success
-- Performing Test SUPPORTS_VARIADIC_MACROS_FLAG
-- Performing Test SUPPORTS_VARIADIC_MACROS_FLAG - Success
-- Performing Test SUPPORTS_GNU_ZERO_VARIADIC_MACRO_ARGUMENTS_FLAG
-- Performing Test SUPPORTS_GNU_ZERO_VARIADIC_MACRO_ARGUMENTS_FLAG - Failed
-- Performing Test HAS_MAYBE_UNINITIALIZED
-- Performing Test HAS_MAYBE_UNINITIALIZED - Success
-- Native target architecture is Sparc
-- Threads enabled.
-- Doxygen disabled.
-- Go bindings disabled.
-- Ninja version: 1.9.0
-- Could NOT find OCaml (missing: OCAMLFIND OCAML_VERSION OCAML_STDLIB_PATH) 
-- OCaml bindings disabled.
-- Could NOT find Python module pygments
-- Could NOT find Python module pygments.lexers.c_cpp
-- Could NOT find Python module yaml
-- LLVM host triple: sparc-sun-solaris2.11
-- LLVM default target triple: sparcv9-sun-solaris
-- Performing Test C_SUPPORTS_FPIC
-- Performing Test C_SUPPORTS_FPIC - Success
-- Performing Test CXX_SUPPORTS_FPIC
-- Performing Test CXX_SUPPORTS_FPIC - Success
-- Building with -fPIC
-- Performing Test C_SUPPORTS_FNO_SEMANTIC_INTERPOSITION
-- Performing Test C_SUPPORTS_FNO_SEMANTIC_INTERPOSITION - Success
-- Performing Test CXX_SUPPORTS_FNO_SEMANTIC_INTERPOSITION
-- Performing Test CXX_SUPPORTS_FNO_SEMANTIC_INTERPOSITION - Success
-- Performing Test SUPPORTS_FVISIBILITY_INLINES_HIDDEN_FLAG
-- Performing Test SUPPORTS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
-- Performing Test C_SUPPORTS_WERROR_DATE_TIME
-- Performing Test C_SUPPORTS_WERROR_DATE_TIME - Success
-- Performing Test CXX_SUPPORTS_WERROR_DATE_TIME
-- Performing Test CXX_SUPPORTS_WERROR_DATE_TIME - Success
-- Performing Test CXX_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW
-- Performing Test CXX_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Failed
-- Performing Test CXX_SUPPORTS_MISSING_FIELD_INITIALIZERS_FLAG
-- Performing Test CXX_SUPPORTS_MISSING_FIELD_INITIALIZERS_FLAG - Success
-- Performing Test C_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG
-- Performing Test C_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG - Failed
-- Performing Test CXX_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG
-- Performing Test CXX_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG - Failed
-- Performing Test C_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG
-- Performing Test C_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG - Success
-- Performing Test CXX_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG
-- Performing Test CXX_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG - Success
-- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
-- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Failed
-- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
-- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Failed
-- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG
-- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG - Success
-- Performing Test CXX_SUPPORTS_REDUNDANT_MOVE_FLAG
-- Performing Test CXX_SUPPORTS_REDUNDANT_MOVE_FLAG - Success
-- Performing Test CXX_SUPPORTS_PESSIMIZING_MOVE_FLAG
-- Performing Test CXX_SUPPORTS_PESSIMIZING_MOVE_FLAG - Success
-- Performing Test CXX_SUPPORTS_NOEXCEPT_TYPE_FLAG
-- Performing Test CXX_SUPPORTS_NOEXCEPT_TYPE_FLAG - Success
-- Performing Test CXX_WONT_WARN_ON_FINAL_NONVIRTUALDTOR
-- Performing Test CXX_WONT_WARN_ON_FINAL_NONVIRTUALDTOR - Failed
-- Performing Test CXX_SUPPORTS_SUGGEST_OVERRIDE_FLAG
-- Performing Test CXX_SUPPORTS_SUGGEST_OVERRIDE_FLAG - Success
-- Performing Test CXX_WSUGGEST_OVERRIDE_ALLOWS_ONLY_FINAL
-- Performing Test CXX_WSUGGEST_OVERRIDE_ALLOWS_ONLY_FINAL - Success
-- Performing Test C_WCOMMENT_ALLOWS_LINE_WRAP
-- Performing Test C_WCOMMENT_ALLOWS_LINE_WRAP - Failed
-- Performing Test C_SUPPORTS_STRING_CONVERSION_FLAG
-- Performing Test C_SUPPORTS_STRING_CONVERSION_FLAG - Failed
-- Performing Test CXX_SUPPORTS_STRING_CONVERSION_FLAG
-- Performing Test CXX_SUPPORTS_STRING_CONVERSION_FLAG - Failed
-- Performing Test C_SUPPORTS_MISLEADING_INDENTATION_FLAG
-- Performing Test C_SUPPORTS_MISLEADING_INDENTATION_FLAG - Success
-- Performing Test CXX_SUPPORTS_MISLEADING_INDENTATION_FLAG
-- Performing Test CXX_SUPPORTS_MISLEADING_INDENTATION_FLAG - Success
-- Performing Test LINKER_SUPPORTS_COLOR_DIAGNOSTICS
-- Performing Test LINKER_SUPPORTS_COLOR_DIAGNOSTICS - Failed
-- Performing Test C_SUPPORTS_FNO_FUNCTION_SECTIONS
-- Performing Test C_SUPPORTS_FNO_FUNCTION_SECTIONS - Success
-- Performing Test C_SUPPORTS_FFUNCTION_SECTIONS
-- Performing Test C_SUPPORTS_FFUNCTION_SECTIONS - Success
-- Performing Test CXX_SUPPORTS_FFUNCTION_SECTIONS
-- Performing Test CXX_SUPPORTS_FFUNCTION_SECTIONS - Success
-- Performing Test C_SUPPORTS_FDATA_SECTIONS
-- Performing Test C_SUPPORTS_FDATA_SECTIONS - Success
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Success
-- Looking for os_signpost_interval_begin
-- Looking for os_signpost_interval_begin - not found
-- Found Python3: /usr/bin/python3.9 (found suitable version "3.9.9", minimum required is "3.0") found components: Interpreter 
-- Linker detection: Solaris ld
-- Setting native build dir to /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build/NATIVE
-- Performing Test LINKER_SUPPORTS_Z_DISCARD_UNUSED
-- Performing Test LINKER_SUPPORTS_Z_DISCARD_UNUSED - Success
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- Found Git: /usr/bin/git (found version "2.35.1") 
-- LLVMHello ignored -- Loadable modules not supported on this platform.
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
-- Targeting Hexagon
-- Targeting MSP430
-- Targeting Mips
-- Targeting NVPTX
-- Targeting PowerPC
-- Targeting RISCV
-- Targeting Sparc
-- Targeting SystemZ
-- Targeting WebAssembly
-- Targeting X86
-- Targeting AVR
-- Targeting M68k
-- BugpointPasses ignored -- Loadable modules not supported on this platform.
-- Configuring done
-- Generating done
-- Build files have been written to: /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "256"
[1/2924] Creating /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build/NATIVE...
[2/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ABIBreak.cpp.o
[3/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/AutoConvert.cpp.o
[4/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/COM.cpp.o
[5/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ExtensibleRTTI.cpp.o
[6/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Valgrind.cpp.o
[7/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Watchdog.cpp.o
[8/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Atomic.cpp.o
[9/2924] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regstrlcpy.c.o
[10/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/Types.cpp.o
[11/2924] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regfree.c.o
[12/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MemAlloc.cpp.o
[13/2924] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regerror.c.o
[14/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86ModRMFilters.cpp.o
[15/2924] Generating VCSRevision.h
[16/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MathExtras.cpp.o
[17/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeCaseFold.cpp.o
[18/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BlockFrequency.cpp.o
[19/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BuryPointer.cpp.o
[20/2924] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/Demangle.cpp.o
[21/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Minidump.cpp.o
[22/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCCodeEmitter.cpp.o
[23/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ConvertUTF.cpp.o
[24/2924] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/DLangDemangle.cpp.o
[25/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Locale.cpp.o
[26/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Hashing.cpp.o
[27/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DivisionByConstantInfo.cpp.o
[28/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Signposts.cpp.o
[29/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackendSkeleton.cpp.o
[30/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SmallPtrSet.cpp.o
[31/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringMap.cpp.o
[32/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/IntEqClasses.cpp.o
[33/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Errno.cpp.o
[34/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMBuildAttrs.cpp.o
[35/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/CRC.cpp.o
[36/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/LEB128.cpp.o
[37/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/COFF.cpp.o
[38/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SystemUtils.cpp.o
[39/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/xxhash.cpp.o
[40/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/RISCVAttributes.cpp.o
[41/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/circular_raw_ostream.cpp.o
[42/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MemoryBufferRef.cpp.o
[43/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/InstructionCost.cpp.o
[44/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MSP430Attributes.cpp.o
[45/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMWinEH.cpp.o
[46/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Optional.cpp.o
[47/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Allocator.cpp.o
[48/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/IntervalMap.cpp.o
[49/2924] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/RustDemangle.cpp.o
[50/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DJB.cpp.o
[51/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APSInt.cpp.o
[52/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackend.cpp.o
[53/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ELFAttributes.cpp.o
[54/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_os_ostream.cpp.o
[55/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ConvertUTFWrapper.cpp.o
[56/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SmallVector.cpp.o
[57/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BranchProbability.cpp.o
[58/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/InitLLVM.cpp.o
[59/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoGOFF.cpp.o
[60/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/LineIterator.cpp.o
[61/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Chrono.cpp.o
[62/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FormattedStream.cpp.o
[63/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoCOFF.cpp.o
[64/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoWasm.cpp.o
[65/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Wasm.cpp.o
[66/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/LowLevelType.cpp.o
[67/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FormatVariadic.cpp.o
[68/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/VersionTuple.cpp.o
[69/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Unicode.cpp.o
[70/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/OptimizedStructLayout.cpp.o
[71/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Regex.cpp.o
[72/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[73/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/PrettyStackTrace.cpp.o
[74/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ThreadLocal.cpp.o
[75/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringSaver.cpp.o
[76/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagEdge.cpp.o
[77/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmMacro.cpp.o
[78/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackWriter.cpp.o
[79/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/RWMutex.cpp.o
[80/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Z3Solver.cpp.o
[81/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FoldingSet.cpp.o
[82/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/NativeFormatting.cpp.o
[83/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Twine.cpp.o
[84/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Compression.cpp.o
[85/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Debug.cpp.o
[86/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SHA1.cpp.o
[87/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/TypeSize.cpp.o
[88/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/TrigramIndex.cpp.o
[89/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringExtras.cpp.o
[90/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Threading.cpp.o
[91/2924] Building CXX object lib/Extensions/CMakeFiles/LLVMExtensions.dir/Extensions.cpp.o
[92/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamError.cpp.o
[93/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DeltaAlgorithm.cpp.o
[94/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/PluginLoader.cpp.o
[95/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MD5.cpp.o
[96/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SuffixTree.cpp.o
[97/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/CrashRecoveryContext.cpp.o
[98/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/HardwareUnit.cpp.o
[99/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CodeViewError.cpp.o
[100/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoXCOFF.cpp.o
[101/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MSP430AttributeParser.cpp.o
[102/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/ELF.cpp.o
[103/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/Formatters.cpp.o
[104/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsection.cpp.o
[105/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ManagedStatic.cpp.o
[106/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCLabel.cpp.o
[107/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/X86TargetParser.cpp.o
[108/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DynamicLibrary.cpp.o
[109/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ScaledNumber.cpp.o
[110/2924] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regexec.c.o
[111/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SymbolRemappingReader.cpp.o
[112/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/XCOFF.cpp.o
[113/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/StringMatcher.cpp.o
[114/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoDarwin.cpp.o
[115/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/RISCVAttributeParser.cpp.o
[116/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SHA256.cpp.o
[117/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Magic.cpp.o
[118/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackReader.cpp.o
[119/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ToolOutputFile.cpp.o
[120/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[121/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MachO.cpp.o
[122/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/RandomNumberGenerator.cpp.o
[123/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DataExtractor.cpp.o
[124/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSymbolsSubsection.cpp.o
[125/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsectionRecord.cpp.o
[126/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ThreadPool.cpp.o
[127/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/AArch64TargetParser.cpp.o
[128/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSymbolRVASubsection.cpp.o
[129/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/GlobPattern.cpp.o
[130/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInst.cpp.o
[131/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstrAnalysis.cpp.o
[132/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCTargetOptions.cpp.o
[133/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCValue.cpp.o
[134/2924] Building C object lib/Support/CMakeFiles/LLVMSupport.dir/regcomp.c.o
[135/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCELFObjectTargetWriter.cpp.o
[136/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamRef.cpp.o
[137/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.o
[138/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.o
[139/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstrDesc.cpp.o
[140/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/SDNodeProperties.cpp.o
[141/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWasmObjectTargetWriter.cpp.o
[142/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.o
[143/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/TarWriter.cpp.o
[144/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[145/2924] Creating export file for LTO
[146/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCXCOFFObjectTargetWriter.cpp.o
[147/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/TargetParser.cpp.o
[148/2924] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
[149/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Memory.cpp.o
[150/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamWriter.cpp.o
[151/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileUtilities.cpp.o
[152/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/AppendingTypeTableBuilder.cpp.o
[153/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/Line.cpp.o
[154/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/OptEmitter.cpp.o
[155/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstrInfo.cpp.o
[156/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/AMDGPUMetadataVerifier.cpp.o
[157/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/WithColor.cpp.o
[158/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/OptRSTEmitter.cpp.o
[159/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCRegisterInfo.cpp.o
[160/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSymbolELF.cpp.o
[161/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileOutputBuffer.cpp.o
[162/2924] Creating export file for Remarks
[163/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstPrinter.cpp.o
[164/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/GraphWriter.cpp.o
[165/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugCrossExSubsection.cpp.o
[166/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSymbolXCOFF.cpp.o
[167/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/Attributes.cpp.o
[168/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
[169/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DebugCounter.cpp.o
[170/2924] Building C object utils/count/CMakeFiles/count.dir/count.c.o
[171/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugChecksumsSubsection.cpp.o
[172/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamReader.cpp.o
[173/2924] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.o
[174/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Process.cpp.o
[175/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugStringTableSubsection.cpp.o
[176/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionCOFF.cpp.o
[177/2924] Linking C executable bin/count
[178/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CTagsEmitter.cpp.o
[179/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmBackend.cpp.o
[180/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/KnownBits.cpp.o
[181/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugFrameDataSubsection.cpp.o
[182/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CodeViewRecordIO.cpp.o
[183/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APFixedPoint.cpp.o
[184/2924] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/MCSymbolizer.cpp.o
[185/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSchedule.cpp.o
[186/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringRef.cpp.o
[187/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMTargetParser.cpp.o
[188/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionXCOFF.cpp.o
[189/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionWasm.cpp.o
[190/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[191/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/CachePruning.cpp.o
[192/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/EnumTables.cpp.o
[193/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugInlineeLinesSubsection.cpp.o
[194/2924] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/MCDisassembler.cpp.o
[195/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Program.cpp.o
[196/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackDocument.cpp.o
[197/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/SubtargetFeatureInfo.cpp.o
[198/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/ContinuationRecordBuilder.cpp.o
[199/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/SubtargetFeature.cpp.o
[200/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/LockFileManager.cpp.o
[201/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Host.cpp.o
[202/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackDocumentYAML.cpp.o
[203/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Caching.cpp.o
[204/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/MemoryBuffer.cpp.o
[205/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/CodeGenCoverage.cpp.o
[206/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenHwModes.cpp.o
[207/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugLinesSubsection.cpp.o
[208/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_ostream.cpp.o
[209/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMAttributeParser.cpp.o
[210/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/GlobalTypeTableBuilder.cpp.o
[211/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmLexer.cpp.o
[212/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.o
[213/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/StringTableBuilder.cpp.o
[214/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SourceMgr.cpp.o
[215/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugCrossImpSubsection.cpp.o
[216/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagInstr.cpp.o
[217/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCMachObjectTargetWriter.cpp.o
[218/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionELF.cpp.o
[219/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSubtargetInfo.cpp.o
[220/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Parallel.cpp.o
[221/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCTargetOptionsCommandFlags.cpp.o
[222/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileCollector.cpp.o
[223/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/CodeEmitter.cpp.o
[224/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsectionVisitor.cpp.o
[225/2924] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/MCRelocationInfo.cpp.o
[226/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/ExegesisEmitter.cpp.o
[227/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[228/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/DetailedRecordsBackend.cpp.o
[229/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
[230/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/TargetRegistry.cpp.o
[231/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HWEventListener.cpp.o
[232/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.o
[233/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCLinkerOptimizationHint.cpp.o
[234/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/TableGen.cpp.o
[235/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Support.cpp.o
[236/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ELFAttributeParser.cpp.o
[237/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCObjectWriter.cpp.o
[238/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Statistic.cpp.o
[239/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86DisassemblerTables.cpp.o
[240/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGLexer.cpp.o
[241/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/LSUnit.cpp.o
[242/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CVSymbolVisitor.cpp.o
[243/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/ArchitectureSet.cpp.o
[244/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoELF.cpp.o
[245/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DisassemblerEmitter.cpp.o
[246/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmParser.cpp.o
[247/2924] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/Arg.cpp.o
[248/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/MicroOpQueueStage.cpp.o
[249/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/PackedVersion.cpp.o
[250/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCNullStreamer.cpp.o
[251/2924] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/Option.cpp.o
[252/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ScopedPrinter.cpp.o
[253/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDag.cpp.o
[254/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeBeadsGen.cpp.o
[255/2924] Building CXX object lib/Debuginfod/CMakeFiles/LLVMDebuginfod.dir/DIFetcher.cpp.o
[256/2924] Building CXX object lib/Bitstream/Reader/CMakeFiles/LLVMBitstreamReader.dir/BitstreamReader.cpp.o
[257/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/AsmLexer.cpp.o
[258/2924] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MSFError.cpp.o
[259/2924] Building CXX object utils/PerfectShuffle/CMakeFiles/llvm-PerfectShuffle.dir/PerfectShuffle.cpp.o
[260/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/PredicateExpander.cpp.o
[261/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/Stage.cpp.o
[262/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/SpecialCaseList.cpp.o
[263/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
[264/2924] Linking CXX executable bin/llvm-PerfectShuffle
[265/2924] Building CXX object lib/LineEditor/CMakeFiles/LLVMLineEditor.dir/LineEditor.cpp.o
[266/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Triple.cpp.o
[267/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Signals.cpp.o
[268/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[269/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/EntryStage.cpp.o
[270/2924] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MSFCommon.cpp.o
[271/2924] Building CXX object lib/Debuginfod/CMakeFiles/LLVMDebuginfod.dir/HTTPClient.cpp.o
[272/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/RetireStage.cpp.o
[273/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/TimeProfiler.cpp.o
[274/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/AsmWriterInst.cpp.o
[275/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[276/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Architecture.cpp.o
[277/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/ExecuteStage.cpp.o
[278/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/InstructionTables.cpp.o
[279/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Target.cpp.o
[280/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/View.cpp.o
[281/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/OptParserEmitter.cpp.o
[282/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLTraits.cpp.o
[283/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/RISCVISAInfo.cpp.o
[284/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCFragment.cpp.o
[285/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/Scheduler.cpp.o
[286/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RegisterFile.cpp.o
[287/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CallingConvEmitter.cpp.o
[288/2924] Building CXX object lib/WindowsManifest/CMakeFiles/LLVMWindowsManifest.dir/WindowsManifestMerger.cpp.o
[289/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Symbol.cpp.o
[290/2924] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/ArgList.cpp.o
[291/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeTableCollection.cpp.o
[292/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/CustomBehaviour.cpp.o
[293/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APInt.cpp.o
[294/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeHashing.cpp.o
[295/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
[296/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcherOpt.cpp.o
[297/2924] Building CXX object utils/not/CMakeFiles/not.dir/not.cpp.o
[298/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/DispatchStage.cpp.o
[299/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSection.cpp.o
[300/2924] Building CXX object lib/Debuginfod/CMakeFiles/LLVMDebuginfod.dir/Debuginfod.cpp.o
[301/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/InfoByHwMode.cpp.o
[302/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/ConstantPools.cpp.o
[303/2924] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Dwarf.cpp.o
[304/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/ResourceManager.cpp.o
[305/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Platform.cpp.o
[306/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/MergingTypeTableBuilder.cpp.o
[307/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordHelpers.cpp.o
[308/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86EVEX2VEXTablesEmitter.cpp.o
[309/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/StringsAndChecksums.cpp.o
[310/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Path.cpp.o
[311/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSymbol.cpp.o
[312/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLParser.cpp.o
[313/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Timer.cpp.o
[314/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolSerializer.cpp.o
[315/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfo.cpp.o
[316/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/JSON.cpp.o
[317/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[318/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordSerialization.cpp.o
[319/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CVTypeVisitor.cpp.o
[320/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/RegisterBankEmitter.cpp.o
[321/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionMachO.cpp.o
[322/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86FoldTablesEmitter.cpp.o
[323/2924] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/OptTable.cpp.o
[324/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndex.cpp.o
[325/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmParserExtension.cpp.o
[326/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/PseudoLoweringEmitter.cpp.o
[327/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWinEH.cpp.o
[328/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCXCOFFStreamer.cpp.o
[329/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcher.cpp.o
[330/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCTargetAsmParser.cpp.o
[331/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/InOrderIssueStage.cpp.o
[332/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/InstrDocsEmitter.cpp.o
[333/2924] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MappedBlockStream.cpp.o
[334/2924] Building CXX object tools/llvm-debuginfod-find/CMakeFiles/llvm-debuginfod-find.dir/llvm-debuginfod-find.cpp.o
[335/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelEmitter.cpp.o
[336/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/GOFFAsmParser.cpp.o
[337/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/COFFMasmParser.cpp.o
[338/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeStreamMerger.cpp.o
[339/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCObjectFileInfo.cpp.o
[340/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SimpleTypeSerializer.cpp.o
[341/2924] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/MCExternalSymbolizer.cpp.o
[342/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordName.cpp.o
[343/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCExpr.cpp.o
[344/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenInstruction.cpp.o
[345/2924] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MSFBuilder.cpp.o
[346/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APFloat.cpp.o
[347/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/TextStubCommon.cpp.o
[348/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/XCOFFAsmParser.cpp.o
[349/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolRecordMapping.cpp.o
[350/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndexDiscovery.cpp.o
[351/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DirectiveEmitter.cpp.o
[352/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCELFStreamer.cpp.o
[353/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWin64EH.cpp.o
[354/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenMapTable.cpp.o
[355/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Context.cpp.o
[356/2924] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[357/2924] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/InstrBuilder.cpp.o
[358/2924] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o
[359/2924] Building CXX object tools/split-file/CMakeFiles/split-file.dir/split-file.cpp.o
[360/2924] Linking CXX static library lib/libLLVMDemangle.a
[361/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWinCOFFStreamer.cpp.o
[362/2924] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.o
[363/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeDumpVisitor.cpp.o
[364/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWasmStreamer.cpp.o
[365/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/COFFAsmParser.cpp.o
[366/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/WasmAsmParser.cpp.o
[367/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCCodeView.cpp.o
[368/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/InterfaceFile.cpp.o
[369/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/LazyRandomTypeCollection.cpp.o
[370/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DFAPacketizerEmitter.cpp.o
[371/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAssembler.cpp.o
[372/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCMachOStreamer.cpp.o
[373/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/XCOFFObjectWriter.cpp.o
[374/2924] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/Disassembler.cpp.o
[375/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeEmitterGen.cpp.o
[376/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcherGen.cpp.o
[377/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/WinCOFFObjectWriter.cpp.o
[378/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CompressInstEmitter.cpp.o
[379/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/ELFAsmParser.cpp.o
[380/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolRecordHelpers.cpp.o
[381/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DFAEmitter.cpp.o
[382/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCObjectStreamer.cpp.o
[383/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MachObjectWriter.cpp.o
[384/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCPseudoProbe.cpp.o
[385/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/IntrinsicEmitter.cpp.o
[386/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/SearchableTableEmitter.cpp.o
[387/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/ELFObjectWriter.cpp.o
[388/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/DarwinAsmParser.cpp.o
[389/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordMapping.cpp.o
[390/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcherEmitter.cpp.o
[391/2924] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolDumper.cpp.o
[392/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/CommandLine.cpp.o
[393/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGParser.cpp.o
[394/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/VirtualFileSystem.cpp.o
[395/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCStreamer.cpp.o
[396/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCDwarf.cpp.o
[397/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmStreamer.cpp.o
[398/2924] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
[399/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/SubtargetEmitter.cpp.o
[400/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenTarget.cpp.o
[401/2924] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[402/2924] Building CXX object utils/FileCheck/CMakeFiles/FileCheck.dir/FileCheck.cpp.o
[403/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/InstrInfoEmitter.cpp.o
[404/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/FastISelEmitter.cpp.o
[405/2924] Building CXX object lib/FileCheck/CMakeFiles/LLVMFileCheck.dir/FileCheck.cpp.o
[406/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/FixedLenDecoderEmitter.cpp.o
[407/2924] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ItaniumManglingCanonicalizer.cpp.o
[408/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/AsmWriterEmitter.cpp.o
[409/2924] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/TextStub.cpp.o
[410/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCContext.cpp.o
[411/2924] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/WasmObjectWriter.cpp.o
[412/2924] Linking CXX static library lib/libLLVMSupport.a
[413/2924] Linking CXX static library lib/libLLVMExtensions.a
[414/2924] Linking CXX static library lib/libLLVMBitstreamReader.a
[415/2924] Linking CXX static library lib/libLLVMWindowsManifest.a
[416/2924] Linking CXX static library lib/libLLVMOption.a
[417/2924] Linking CXX static library lib/libLLVMLineEditor.a
[418/2924] Linking CXX static library lib/libLLVMDebuginfod.a
[419/2924] Linking CXX static library lib/libLLVMDebugInfoMSF.a
[420/2924] Linking CXX static library lib/libLLVMFileCheck.a
[421/2924] Linking CXX static library lib/libLLVMBinaryFormat.a
[422/2924] Linking CXX static library lib/libLLVMTableGen.a
[423/2924] Linking CXX static library lib/libLLVMTextAPI.a
[424/2924] Linking CXX static library lib/libLLVMTableGenGlobalISel.a
[425/2924] Linking CXX executable bin/yaml-bench
[426/2924] Linking CXX executable bin/split-file
[427/2924] Linking CXX static library lib/libLLVMDebugInfoCodeView.a
[428/2924] Linking CXX executable bin/llvm-debuginfod-find
[429/2924] Linking CXX executable bin/not
[430/2924] Linking CXX executable bin/FileCheck
[431/2924] Linking CXX executable bin/llvm-undname
[432/2924] Linking CXX static library lib/libLLVMMC.a
[433/2924] Linking CXX static library lib/libLLVMMCDisassembler.a
[434/2924] Linking CXX static library lib/libLLVMMCA.a
[435/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/GICombinerEmitter.cpp.o
[436/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenSchedule.cpp.o
[437/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/RegisterInfoEmitter.cpp.o
[438/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenRegisters.cpp.o
[439/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[440/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/AsmParser.cpp.o
[441/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenDAGPatterns.cpp.o
[442/2924] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MasmParser.cpp.o
[443/2924] Linking CXX static library lib/libLLVMMCParser.a
[444/2924] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/GlobalISelEmitter.cpp.o
[445/2924] Linking CXX executable bin/llvm-tblgen
[446/2924] Configuring NATIVE LLVM...
FAILED: NATIVE/CMakeCache.txt /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build/NATIVE/CMakeCache.txt 
cd /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build/NATIVE && /usr/bin/cmake -G Ninja -DCMAKE_MAKE_PROGRAM="/usr/bin/ninja" -DCMAKE_C_COMPILER_LAUNCHER="" -DCMAKE_CXX_COMPILER_LAUNCHER="" -DCMAKE_C_COMPILER=/builds/psumbera/RUST/solaris-userland/components/rust/rustc/epoll/gcc -DCMAKE_CXX_COMPILER=/usr/bin/g++ /builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/src/llvm-project/llvm -DLLVM_TARGET_IS_CROSSCOMPILE_HOST=TRUE -DLLVM_TARGETS_TO_BUILD="AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86;AVR;M68k" -DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD="AVR;M68k" -DLLVM_DEFAULT_TARGET_TRIPLE="sparcv9-sun-solaris" -DLLVM_TARGET_ARCH="sparcv9" -DLLVM_ENABLE_PROJECTS="" -DLLVM_EXTERNAL_PROJECTS="" -DLLVM_ENABLE_RUNTIMES="" -DLLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN="OFF" -DCMAKE_BUILD_TYPE=Release
-- The C compiler identification is GNU 11.2.0
-- The CXX compiler identification is GNU 10.3.0
-- The ASM compiler identification is GNU
-- Found assembler: /builds/psumbera/RUST/solaris-userland/components/rust/rustc/epoll/gcc
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: /builds/psumbera/RUST/solaris-userland/components/rust/rustc/epoll/gcc - skipped
-- Detecting C compile features
-- Detecting C compile features - done
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Check for working CXX compiler: /usr/bin/g++ - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Looking for dlfcn.h
-- Looking for dlfcn.h - found
-- Looking for errno.h
-- Looking for errno.h - found
-- Looking for fcntl.h
-- Looking for fcntl.h - found
-- Looking for link.h
-- Looking for link.h - found
-- Looking for malloc/malloc.h
-- Looking for malloc/malloc.h - not found
-- Looking for pthread.h
-- Looking for pthread.h - found
-- Looking for signal.h
-- Looking for signal.h - found
-- Looking for sys/ioctl.h
-- Looking for sys/ioctl.h - found
-- Looking for sys/mman.h
-- Looking for sys/mman.h - found
-- Looking for sys/param.h
-- Looking for sys/param.h - found
-- Looking for sys/resource.h
-- Looking for sys/resource.h - found
-- Looking for sys/stat.h
-- Looking for sys/stat.h - found
-- Looking for sys/time.h
-- Looking for sys/time.h - found
-- Looking for sys/types.h
-- Looking for sys/types.h - found
-- Looking for sysexits.h
-- Looking for sysexits.h - found
-- Looking for termios.h
-- Looking for termios.h - found
-- Looking for unistd.h
-- Looking for unistd.h - found
-- Looking for valgrind/valgrind.h
-- Looking for valgrind/valgrind.h - not found
-- Looking for fenv.h
-- Looking for fenv.h - found
-- Looking for FE_ALL_EXCEPT
-- Looking for FE_ALL_EXCEPT - found
-- Looking for FE_INEXACT
-- Looking for FE_INEXACT - found
-- Looking for mach/mach.h
-- Looking for mach/mach.h - not found
-- Looking for histedit.h
-- Looking for histedit.h - found
-- Looking for CrashReporterClient.h
-- Looking for CrashReporterClient.h - not found
-- Looking for pthread_create in pthread
-- Looking for pthread_create in pthread - found
-- Looking for pthread_getspecific in pthread
-- Looking for pthread_getspecific in pthread - found
-- Looking for pthread_rwlock_init in pthread
-- Looking for pthread_rwlock_init in pthread - found
-- Looking for pthread_mutex_lock in pthread
-- Looking for pthread_mutex_lock in pthread - found
-- Looking for dlopen in dl
-- Looking for dlopen in dl - found
-- Looking for clock_gettime in rt
-- Looking for clock_gettime in rt - found
-- Looking for pfm_initialize in pfm
-- Looking for pfm_initialize in pfm - not found
-- Looking for pthread.h
-- Looking for pthread.h - found
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD - Success
-- Found Threads: TRUE  
-- Found ZLIB: /usr/lib/64/libz.so (found version "1.2.11") 
-- Looking for compress2
-- Looking for compress2 - found
-- Found LibXml2: /usr/lib/sparcv9/libxml2.so (found version "2.9.12") 
-- Looking for xmlReadMemory
-- Looking for xmlReadMemory - found
-- Looking for el_init in edit
-- Looking for el_init in edit - found
-- Performing Test Terminfo_LINKABLE
-- Performing Test Terminfo_LINKABLE - Success
-- Found Terminfo: /usr/lib/64/libcurses.so  
-- Looking for xar_open in xar
-- Looking for xar_open in xar - not found
-- Looking for arc4random
-- Looking for arc4random - found
-- Looking for backtrace
-- Looking for backtrace - found
-- backtrace facility detected in default set of libraries
-- Found Backtrace: /usr/include  
-- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW
-- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Failed
-- Looking for __register_frame
-- Looking for __register_frame - found
-- Looking for __deregister_frame
-- Looking for __deregister_frame - found
-- Looking for __unw_add_dynamic_fde
-- Looking for __unw_add_dynamic_fde - not found
-- Looking for _Unwind_Backtrace
-- Looking for _Unwind_Backtrace - found
-- Looking for getpagesize
-- Looking for getpagesize - found
-- Looking for sysconf
-- Looking for sysconf - found
-- Looking for getrusage
-- Looking for getrusage - found
-- Looking for setrlimit
-- Looking for setrlimit - found
-- Looking for isatty
-- Looking for isatty - found
-- Looking for futimens
-- Looking for futimens - found
-- Looking for futimes
-- Looking for futimes - not found
-- Looking for sigaltstack
-- Looking for sigaltstack - found
-- Looking for lseek64
-- Looking for lseek64 - found
-- Looking for mallctl
-- Looking for mallctl - not found
-- Looking for mallinfo
-- Looking for mallinfo - not found
-- Looking for mallinfo2
-- Looking for mallinfo2 - not found
-- Looking for malloc_zone_statistics
-- Looking for malloc_zone_statistics - not found
-- Looking for getrlimit
-- Looking for getrlimit - found
-- Looking for posix_spawn
-- Looking for posix_spawn - found
-- Looking for pread
-- Looking for pread - found
-- Looking for sbrk
-- Looking for sbrk - found
-- Looking for strerror
-- Looking for strerror - found
-- Looking for strerror_r
-- Looking for strerror_r - found
-- Looking for strerror_s
-- Looking for strerror_s - not found
-- Looking for setenv
-- Looking for setenv - found
-- Looking for dlopen
-- Looking for dlopen - found
-- Looking for dladdr
-- Looking for dladdr - found
-- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC
-- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC - Failed
-- Performing Test HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC
-- Performing Test HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC - Success
-- Looking for __GLIBC__
-- Looking for __GLIBC__ - not found
-- Looking for pthread_getname_np
-- Looking for pthread_getname_np - found
-- Looking for pthread_setname_np
-- Looking for pthread_setname_np - found
-- Looking for proc_pid_rusage
-- Looking for proc_pid_rusage - not found
-- Performing Test HAVE_STD_IS_TRIVIALLY_COPYABLE
-- Performing Test HAVE_STD_IS_TRIVIALLY_COPYABLE - Success
-- Performing Test HAVE_CXX_ATOMICS_WITHOUT_LIB
-- Performing Test HAVE_CXX_ATOMICS_WITHOUT_LIB - Success
-- Performing Test HAVE_CXX_ATOMICS64_WITHOUT_LIB
-- Performing Test HAVE_CXX_ATOMICS64_WITHOUT_LIB - Success
-- Performing Test LLVM_HAS_ATOMICS
-- Performing Test LLVM_HAS_ATOMICS - Success
-- Performing Test SUPPORTS_VARIADIC_MACROS_FLAG
-- Performing Test SUPPORTS_VARIADIC_MACROS_FLAG - Success
-- Performing Test SUPPORTS_GNU_ZERO_VARIADIC_MACRO_ARGUMENTS_FLAG
-- Performing Test SUPPORTS_GNU_ZERO_VARIADIC_MACRO_ARGUMENTS_FLAG - Failed
-- Performing Test HAS_MAYBE_UNINITIALIZED
-- Performing Test HAS_MAYBE_UNINITIALIZED - Success
-- Native target architecture is Sparc
-- Threads enabled.
-- Doxygen disabled.
-- Go bindings enabled.
-- Ninja version: 1.9.0
-- Could NOT find OCaml (missing: OCAMLFIND OCAML_VERSION OCAML_STDLIB_PATH) 
-- Could NOT find OCaml (missing: OCAMLFIND OCAML_VERSION OCAML_STDLIB_PATH) 
-- OCaml bindings disabled.
-- Could NOT find Python module pygments
-- Could NOT find Python module pygments.lexers.c_cpp
-- Could NOT find Python module yaml
-- LLVM host triple: sparc-sun-solaris2.11
-- LLVM default target triple: sparcv9-sun-solaris
-- Performing Test C_SUPPORTS_FPIC
-- Performing Test C_SUPPORTS_FPIC - Success
-- Performing Test CXX_SUPPORTS_FPIC
-- Performing Test CXX_SUPPORTS_FPIC - Success
-- Building with -fPIC
-- Performing Test C_SUPPORTS_FNO_SEMANTIC_INTERPOSITION
-- Performing Test C_SUPPORTS_FNO_SEMANTIC_INTERPOSITION - Success
-- Performing Test CXX_SUPPORTS_FNO_SEMANTIC_INTERPOSITION
-- Performing Test CXX_SUPPORTS_FNO_SEMANTIC_INTERPOSITION - Success
-- Performing Test SUPPORTS_FVISIBILITY_INLINES_HIDDEN_FLAG
-- Performing Test SUPPORTS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
-- Performing Test C_SUPPORTS_WERROR_DATE_TIME
-- Performing Test C_SUPPORTS_WERROR_DATE_TIME - Success
-- Performing Test CXX_SUPPORTS_WERROR_DATE_TIME
-- Performing Test CXX_SUPPORTS_WERROR_DATE_TIME - Success
-- Performing Test CXX_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW
-- Performing Test CXX_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Failed
-- Performing Test CXX_SUPPORTS_MISSING_FIELD_INITIALIZERS_FLAG
-- Performing Test CXX_SUPPORTS_MISSING_FIELD_INITIALIZERS_FLAG - Success
-- Performing Test C_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG
-- Performing Test C_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG - Failed
-- Performing Test CXX_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG
-- Performing Test CXX_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG - Failed
-- Performing Test C_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG
-- Performing Test C_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG - Success
-- Performing Test CXX_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG
-- Performing Test CXX_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG - Success
-- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
-- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Failed
-- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
-- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Failed
-- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG
-- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG - Success
-- Performing Test CXX_SUPPORTS_REDUNDANT_MOVE_FLAG
-- Performing Test CXX_SUPPORTS_REDUNDANT_MOVE_FLAG - Success
-- Performing Test CXX_SUPPORTS_PESSIMIZING_MOVE_FLAG
-- Performing Test CXX_SUPPORTS_PESSIMIZING_MOVE_FLAG - Success
-- Performing Test CXX_SUPPORTS_NOEXCEPT_TYPE_FLAG
-- Performing Test CXX_SUPPORTS_NOEXCEPT_TYPE_FLAG - Success
-- Performing Test CXX_WONT_WARN_ON_FINAL_NONVIRTUALDTOR
-- Performing Test CXX_WONT_WARN_ON_FINAL_NONVIRTUALDTOR - Failed
-- Performing Test CXX_SUPPORTS_SUGGEST_OVERRIDE_FLAG
-- Performing Test CXX_SUPPORTS_SUGGEST_OVERRIDE_FLAG - Success
-- Performing Test CXX_WSUGGEST_OVERRIDE_ALLOWS_ONLY_FINAL
-- Performing Test CXX_WSUGGEST_OVERRIDE_ALLOWS_ONLY_FINAL - Success
-- Performing Test C_WCOMMENT_ALLOWS_LINE_WRAP
-- Performing Test C_WCOMMENT_ALLOWS_LINE_WRAP - Failed
-- Performing Test C_SUPPORTS_STRING_CONVERSION_FLAG
-- Performing Test C_SUPPORTS_STRING_CONVERSION_FLAG - Failed
-- Performing Test CXX_SUPPORTS_STRING_CONVERSION_FLAG
-- Performing Test CXX_SUPPORTS_STRING_CONVERSION_FLAG - Failed
-- Performing Test C_SUPPORTS_MISLEADING_INDENTATION_FLAG
-- Performing Test C_SUPPORTS_MISLEADING_INDENTATION_FLAG - Success
-- Performing Test CXX_SUPPORTS_MISLEADING_INDENTATION_FLAG
-- Performing Test CXX_SUPPORTS_MISLEADING_INDENTATION_FLAG - Success
-- Performing Test LINKER_SUPPORTS_COLOR_DIAGNOSTICS
-- Performing Test LINKER_SUPPORTS_COLOR_DIAGNOSTICS - Failed
-- Performing Test C_SUPPORTS_FNO_FUNCTION_SECTIONS
-- Performing Test C_SUPPORTS_FNO_FUNCTION_SECTIONS - Success
-- Performing Test C_SUPPORTS_FFUNCTION_SECTIONS
-- Performing Test C_SUPPORTS_FFUNCTION_SECTIONS - Success
-- Performing Test CXX_SUPPORTS_FFUNCTION_SECTIONS
-- Performing Test CXX_SUPPORTS_FFUNCTION_SECTIONS - Success
-- Performing Test C_SUPPORTS_FDATA_SECTIONS
-- Performing Test C_SUPPORTS_FDATA_SECTIONS - Success
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Success
-- Looking for os_signpost_interval_begin
-- Looking for os_signpost_interval_begin - not found
-- Found Python3: /usr/bin/python3.9 (found suitable version "3.9.9", minimum required is "3.6") found components: Interpreter 
-- Linker detection: Solaris ld
-- Performing Test LINKER_SUPPORTS_Z_DISCARD_UNUSED
-- Performing Test LINKER_SUPPORTS_Z_DISCARD_UNUSED - Success
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- Found Git: /usr/bin/git (found version "2.35.1") 
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
-- Targeting Hexagon
-- Targeting MSP430
-- Targeting Mips
-- Targeting NVPTX
-- Targeting PowerPC
-- Targeting RISCV
-- Targeting Sparc
-- Targeting SystemZ
-- Targeting WebAssembly
-- Targeting X86
-- Targeting AVR
-- Targeting M68k
-- Registering Bye as a pass plugin (static build: OFF)
CMake Error at CMakeLists.txt:1256 (add_subdirectory):
  add_subdirectory given source
  "/builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/src/llvm-project/llvm/../third-party/benchmark"
  which is not an existing directory.


-- Configuring incomplete, errors occurred!
See also "/builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build/NATIVE/CMakeFiles/CMakeOutput.log".
See also "/builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/llvm/build/NATIVE/CMakeFiles/CMakeError.log".
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /builds/psumbera/RUST/solaris-userland/components/rust/rustc/rustc-1.60.0-src/vendor/cmake/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	finished in 92.880 seconds
Build completed unsuccessfully in 0:07:30
make[1]: *** [Makefile:12: all] Error 1
make[1]: Leaving directory '/builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9'
gmake: *** [/builds/psumbera/RUST/solaris-userland/make-rules/configure.mk:184: /builds/psumbera/RUST/solaris-userland/components/rust/rustc/build/sparcv9/.built] Error 2
