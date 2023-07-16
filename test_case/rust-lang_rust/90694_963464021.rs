
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/home/infrandomness/src/rust/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-dev" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=clang-13" "-DCMAKE_CXX_COMPILER=clang++-13" "-DCMAKE_ASM_COMPILER=clang-13" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_INSTALL_PREFIX=/home/infrandomness/src/rust/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The C compiler identification is Clang 13.0.1
-- The CXX compiler identification is Clang 13.0.1
-- The ASM compiler identification is Clang
-- Found assembler: /usr/bin/clang-13
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: /usr/bin/clang-13 - skipped
-- Detecting C compile features
-- Detecting C compile features - done
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Check for working CXX compiler: /usr/bin/clang++-13 - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Performing Test LLVM_LIBSTDCXX_MIN
-- Performing Test LLVM_LIBSTDCXX_MIN - Success
-- Performing Test LLVM_LIBSTDCXX_SOFT_ERROR
-- Performing Test LLVM_LIBSTDCXX_SOFT_ERROR - Success
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
-- Looking for histedit.h - not found
-- Looking for CrashReporterClient.h
-- Looking for CrashReporterClient.h - not found
-- Looking for linux/magic.h
-- Looking for linux/magic.h - found
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
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD - Failed
-- Looking for pthread_create in pthreads
-- Looking for pthread_create in pthreads - not found
-- Looking for pthread_create in pthread
-- Looking for pthread_create in pthread - found
-- Found Threads: TRUE
-- Found ZLIB: /usr/lib/x86_64-linux-gnu/libz.so (found version "1.2.11")
-- Looking for compress2
-- Looking for compress2 - found
-- Looking for xar_open in xar
-- Looking for xar_open in xar - not found
-- Looking for arc4random
-- Looking for arc4random - not found
-- Looking for backtrace
-- Looking for backtrace - found
-- backtrace facility detected in default set of libraries
-- Found Backtrace: /usr/include
-- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW
-- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Success
-- Looking for __register_frame
-- Looking for __register_frame - found
-- Looking for __deregister_frame
-- Looking for __deregister_frame - found
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
-- Looking for futimes - found
-- Looking for posix_fallocate
-- Looking for posix_fallocate - found
-- Looking for sigaltstack
-- Looking for sigaltstack - found
-- Looking for lseek64
-- Looking for lseek64 - found
-- Looking for mallctl
-- Looking for mallctl - not found
-- Looking for mallinfo
-- Looking for mallinfo - found
-- Looking for mallinfo2
-- Looking for mallinfo2 - found
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
-- Looking for dladdr - not found
-- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC
-- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC - Failed
-- Performing Test HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC
-- Performing Test HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC - Success
-- Looking for __GLIBC__
-- Looking for __GLIBC__ - found
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
-- Performing Test SUPPORTS_GNU_ZERO_VARIADIC_MACRO_ARGUMENTS_FLAG - Success
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 6:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 8:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 28:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 29:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 40:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 42:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 54:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 65:
: not foundndomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 68:
/home/infrandomness/src/rust/src/llvm-project/llvm/cmake/config.guess: 71: Syntax error: word unexpected (expecting "in")
CMake Error at cmake/modules/GetHostTriple.cmake:36 (message):
  Failed to execute
  /home/infrandomness/src/rust/src/llvm-project/llvm/cmake/config.guess
Call Stack (most recent call first):
  cmake/config-ix.cmake:411 (get_host_triple)
  CMakeLists.txt:684 (include)


-- Configuring incomplete, errors occurred!
See also "/home/infrandomness/src/rust/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/home/infrandomness/src/rust/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /home/infrandomness/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        finished in 13.820 seconds
