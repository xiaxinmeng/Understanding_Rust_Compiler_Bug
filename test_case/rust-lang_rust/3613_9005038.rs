
Adam@WEEDENASUS1 /C/Users/Adam/Documents/GitHub/rust
$ ./configure
configure: looking for configure programs
configure: found cmp
configure: found mkdir
configure: found printf
configure: found cut
configure: found grep
configure: found xargs
configure: found cp
configure: found find
configure: found uname
configure: found date
configure: found tr
configure: found sed
configure: inspecting environment
configure: recreating config.tmp
configure:
configure: processing /C/Users/Adam/Documents/GitHub/rust/configure args
configure:
configure: CFG_PREFIX           := /usr/local
configure: CFG_LOCAL_RUST_ROOT  := /usr/local
configure: CFG_LLVM_ROOT        :=
configure: CFG_HOST_TRIPLE      := i686-pc-mingw32
configure: CFG_TARGET_TRIPLES   := i686-pc-mingw32
configure:
configure: looking for build programs
configure:
configure: CFG_PERL             := /bin/perl.exe
configure: CFG_CURL             := /c/Users/Adam/AppData/Local/GitHub/ ...
configure: CFG_PYTHON           := /c/Python27/python.exe
configure: CFG_GIT              := /c/Users/Adam/AppData/Local/GitHub/ ...
configure: CFG_CLANG            :=
configure: CFG_GCC              := /mingw/bin/gcc.exe (4.6.2)
configure: CFG_LD               := /mingw/bin/ld.exe (2.22)
configure: CFG_LLVM_CONFIG      :=
configure: CFG_VALGRIND         :=
configure: CFG_PERF             :=
configure: CFG_ISCC             :=
configure: CFG_LLNEXTGEN        :=
configure: CFG_PANDOC           :=
configure: CFG_PDFLATEX         :=
configure: CFG_XETEX            :=
configure: CFG_LUATEX           :=
configure: CFG_NODE             :=
configure:
configure: making directories
configure:
configure:
configure: configuring submodules
configure:
configure: git: submodule sync
configure: git: submodule status
 1170ffba3ac5191930b40c897d4569a9d8a296a3 src/libuv (remotes/origin/ipc-listen-2
28-g1170ffb)
 b55be285d18e9b3537fc9d29af44e83be2171326 src/llvm (b55be28)
 853733e772b2885d93fdf994dedc4a1b5dc1369e src/llvm/projects/compiler-rt (853733e
)
 f5e0b225b4d8027edab993ad4ac87510fcd6f991 src/llvm/tools/clang (f5e0b22)
configure: git: submodule update
configure: git: submodule clobber
configure:
configure: looking at LLVM
configure:
configure: configuring LLVM for i686-pc-mingw32
configure: configuring LLVM with:
configure: --enable-targets=x86,x86_64 --enable-optimized --disable-docs
            --enable-bindings=none --disable-threads                    --disabl
e-pthreads --build=i686-pc-mingw32                         --host=i686-pc-mingw3
2 --target=i686-pc-mingw32
checking for i686-pc-mingw32-clang... gcc -m32
checking for C compiler default output file name... a.exe
checking whether the C compiler works... yes
checking whether we are cross compiling... no
checking for suffix of executables... .exe
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc -m32 accepts -g... yes
checking for gcc -m32 option to accept ISO C89... none needed
checking whether we are using the GNU C++ compiler... yes
checking whether g++ -m32 accepts -g... yes
checking how to run the C preprocessor... gcc -m32 -E
checking build system type... i686-pc-mingw32
checking host system type... i686-pc-mingw32
checking target system type... i686-pc-mingw32
checking type of operating system we're going to host on... MingW
checking type of operating system we're going to target... MingW
checking target architecture... x86
checking for grep that handles long lines and -e... /bin/grep
checking for egrep... /bin/grep -E
checking for ANSI C header files... yes
checking for sys/types.h... yes
checking for sys/stat.h... yes
checking for stdlib.h... yes
checking for string.h... yes
checking for memory.h... yes
checking for strings.h... yes
checking for inttypes.h... yes
checking for stdint.h... yes
checking for unistd.h... yes
checking whether byte ordering is bigendian... no
checking optimization flags... -O2
checking for BSD-compatible nm... /mingw/bin/nm
checking for GNU make... make
checking whether ln -s works... no, using cp -p
checking for cmp... /bin/cmp
checking for cp... /bin/cp
checking for date... /bin/date
checking for find... /bin/find
checking for grep... (cached) /bin/grep
checking for mkdir... /bin/mkdir
checking for mv... /bin/mv
checking for i686-pc-mingw32-ranlib... no
checking for ranlib... ranlib
checking for i686-pc-mingw32-ar... no
checking for ar... ar
checking for rm... /bin/rm
checking for sed... /bin/sed
checking for tar... /bin/tar
checking for pwd... /bin/pwd
checking for Graphviz... echo Graphviz
checking for dot... echo dot
checking for fdp... echo fdp
checking for neato... echo neato
checking for twopi... echo twopi
checking for circo... echo circo
checking for gv... no
checking for gsview32... no
checking for dotty... echo dotty
checking for xdot.py... echo xdot.py
checking for a BSD-compatible install... /bin/install -c
checking for bzip2... /bin/bzip2
checking for cat... /bin/cat
checking for doxygen... no
checking for groff... no
checking for gzip... /bin/gzip
checking for pdfroff... no
checking for zip... no
checking for ocamlc... no
checking for ocamlopt... no
checking for ocamldep... no
checking for ocamldoc... no
checking for gas... no
checking for as... /mingw/bin/as
checking for linker version... 2.22
checking for compiler -Wl,-R<path> option... yes
checking for compiler -Wl,-export-dynamic option... yes
checking for compiler -Wl,--version-script option... yes
checking for an ANSI C-conforming const... yes
checking for dirent.h that defines DIR... yes
checking for library containing opendir... none required
checking dlfcn.h usability... no
checking dlfcn.h presence... no
checking for dlfcn.h... no
checking dynamic linker characteristics... Win32 ld.exe
checking which extension is used for loadable modules... .dll
checking which variable specifies run-time library path... PATH
checking for the default library search path... /lib /usr/lib
checking for objdir... .libs
checking command to parse /mingw/bin/nm output from  object... ok
checking whether libtool supports -dlopen/-dlpreopen... yes
checking for shl_load... no
checking for shl_load in -ldld... no
checking for dlopen in -ldl... no
checking for dlopen in -lsvld... no
checking for dld_link in -ldld... no
checking for _dyld_func_lookup... no
checking for _ prefix in compiled symbols... yes
checking whether deplibs are loaded by dlopen... unknown
checking argz.h usability... no
checking argz.h presence... no
checking for argz.h... no
checking for error_t... no
checking for argz_append... no
checking for argz_create_sep... no
checking for argz_insert... no
checking for argz_next... no
checking for argz_stringify... no
checking assert.h usability... yes
checking assert.h presence... yes
checking for assert.h... yes
checking ctype.h usability... yes
checking ctype.h presence... yes
checking for ctype.h... yes
checking errno.h usability... yes
checking errno.h presence... yes
checking for errno.h... yes
checking malloc.h usability... yes
checking malloc.h presence... yes
checking for malloc.h... yes
checking for memory.h... (cached) yes
checking for stdlib.h... (cached) yes
checking stdio.h usability... yes
checking stdio.h presence... yes
checking for stdio.h... yes
checking for unistd.h... (cached) yes
checking dl.h usability... no
checking dl.h presence... no
checking for dl.h... no
checking sys/dl.h usability... no
checking sys/dl.h presence... no
checking for sys/dl.h... no
checking dld.h usability... no
checking dld.h presence... no
checking for dld.h... no
checking mach-o/dyld.h usability... no
checking mach-o/dyld.h presence... no
checking for mach-o/dyld.h... no
checking for string.h... (cached) yes
checking for strchr... yes
checking for strrchr... yes
checking for memcpy... yes
checking for memmove... yes
checking for strcmp... yes
checking for closedir... yes
checking for opendir... yes
checking for readdir... yes
checking tool compatibility... ok
checking optional compiler flags... -Wno-variadic-macros -Wno-missing-field-initializers
checking for sin in -lm... yes
checking for main in -limagehlp... yes
checking for main in -lpsapi... yes
checking for library containing dlopen... no
configure: WARNING: dlopen() not found - disabling plugin support
checking for library containing mallinfo... no
checking for dirent.h that defines DIR... (cached) yes
checking for library containing opendir... (cached) none required
checking for MAP_ANONYMOUS vs. MAP_ANON... no
checking whether stat file-mode macros are broken... no
checking for sys/wait.h that is POSIX.1 compatible... no
checking whether time.h and sys/time.h may both be included... yes
checking for dlfcn.h... (cached) no
checking execinfo.h usability... no
checking execinfo.h presence... no
checking for execinfo.h... no
checking fcntl.h usability... yes
checking fcntl.h presence... yes
checking for fcntl.h... yes
checking for inttypes.h... (cached) yes
checking limits.h usability... yes
checking limits.h presence... yes
checking for limits.h... yes
checking link.h usability... no
checking link.h presence... no
checking for link.h... no
checking for malloc.h... (cached) yes
checking setjmp.h usability... yes
checking setjmp.h presence... yes
checking for setjmp.h... yes
checking signal.h usability... yes
checking signal.h presence... yes
checking for signal.h... yes
checking for stdint.h... (cached) yes
checking termios.h usability... no
checking termios.h presence... no
checking for termios.h... no
checking for unistd.h... (cached) yes
checking utime.h usability... yes
checking utime.h presence... yes
checking for utime.h... yes
checking windows.h usability... yes
checking windows.h presence... yes
checking for windows.h... yes
checking sys/mman.h usability... no
checking sys/mman.h presence... no
checking for sys/mman.h... no
checking sys/param.h usability... yes
checking sys/param.h presence... yes
checking for sys/param.h... yes
checking sys/resource.h usability... no
checking sys/resource.h presence... no
checking for sys/resource.h... no
checking sys/time.h usability... yes
checking sys/time.h presence... yes
checking for sys/time.h... yes
checking sys/uio.h usability... no
checking sys/uio.h presence... no
checking for sys/uio.h... no
checking for sys/types.h... (cached) yes
checking sys/ioctl.h usability... no
checking sys/ioctl.h presence... no
checking for sys/ioctl.h... no
checking malloc/malloc.h usability... no
checking malloc/malloc.h presence... no
checking for malloc/malloc.h... no
checking mach/mach.h usability... no
checking mach/mach.h presence... no
checking for mach/mach.h... no
checking valgrind/valgrind.h usability... no
checking valgrind/valgrind.h presence... no
checking for valgrind/valgrind.h... no
checking fenv.h usability... yes
checking fenv.h presence... yes
checking for fenv.h... yes
checking CrashReporterClient.h usability... no
checking CrashReporterClient.h presence... no
checking for CrashReporterClient.h... no
checking __crashreporter_info__... no
checking for HUGE_VAL sanity... yes
checking for pid_t... yes
checking for size_t... yes
checking whether struct tm is in sys/time.h or time.h... time.h
checking for int64_t... yes
checking for uint64_t... yes
checking for backtrace... no
checking for ceilf... yes
checking for floorf... yes
checking for roundf... yes
checking for rintf... yes
checking for nearbyintf... yes
checking for getcwd... yes
checking for powf... yes
checking for fmodf... yes
checking for strtof... yes
checking for round... yes
checking for getpagesize... yes
checking for getrusage... no
checking for getrlimit... no
checking for setrlimit... no
checking for gettimeofday... yes
checking for isatty... yes
checking for mkdtemp... no
checking for mkstemp... no
checking for mktemp... yes
checking for posix_spawn... no
checking for pread... no
checking for realpath... no
checking for sbrk... no
checking for setrlimit... (cached) no
checking for strdup... yes
checking for strerror... yes
checking for strerror_r... no
checking for setenv... no
checking for arc4random... no
checking for strtoll... yes
checking for strtoq... no
checking for sysconf... no
checking for malloc_zone_statistics... no
checking for setjmp... no
checking for longjmp... yes
checking for sigsetjmp... no
checking for siglongjmp... no
checking for writev... no
checking if printf has the %a format character...
checking for srand48/lrand48/drand48 in <stdlib.h>... no
checking whether strerror_s is declared... no
checking for _alloca in -lgcc... yes
checking for __alloca in -lgcc... no
checking for __chkstk in -lgcc... yes
checking for ___chkstk in -lgcc... no
checking for __ashldi3 in -lgcc... yes
checking for __ashrdi3 in -lgcc... yes
checking for __divdi3 in -lgcc... yes
checking for __fixdfdi in -lgcc... yes
checking for __fixsfdi in -lgcc... yes
checking for __floatdidf in -lgcc... yes
checking for __lshrdi3 in -lgcc... yes
checking for __moddi3 in -lgcc... yes
checking for __udivdi3 in -lgcc... yes
checking for __umoddi3 in -lgcc... yes
checking for __main in -lgcc... yes
checking for __cmpdi2 in -lgcc... yes
checking whether EnumerateLoadedModules() accepts new decl... no
checking for isnan in <math.h>... yes
checking for isnan in <cmath>... no
checking for std::isnan in <cmath>... yes
checking for isinf in <math.h>... yes
checking for isinf in <cmath>... no
checking for std::isinf in <cmath>... yes
checking for finite in <ieeefp.h>... no
checking for GCC atomic builtins... no
configure: WARNING: LLVM will be built thread-unsafe because atomic builtins are missing
checking for __dso_handle... no
checking for compiler -fvisibility-inlines-hidden option... yes
configure: creating ./config.status
config.status: creating include/llvm/Config/Targets.def
config.status: creating include/llvm/Config/AsmPrinters.def
config.status: creating include/llvm/Config/AsmParsers.def
config.status: creating include/llvm/Config/Disassemblers.def
config.status: creating Makefile.config
config.status: creating llvm.spec
config.status: creating docs/doxygen.cfg
config.status: creating tools/clang/docs/doxygen.cfg
config.status: creating bindings/ocaml/llvm/META.llvm
config.status: creating include/llvm/Config/config.h
config.status: creating include/llvm/Config/llvm-config.h
config.status: creating include/llvm/Support/DataTypes.h
config.status: include/llvm/Support/DataTypes.h is unchanged
config.status: creating tools/clang/include/clang/Config/config.h
config.status: tools/clang/include/clang/Config/config.h is unchanged
config.status: executing setup commands
config.status: executing Makefile commands
config.status: executing Makefile.common commands
config.status: executing examples/Makefile commands
config.status: executing lib/Makefile commands
config.status: executing runtime/Makefile commands
config.status: executing test/Makefile commands
config.status: executing test/Makefile.tests commands
config.status: executing unittests/Makefile commands
config.status: executing tools/Makefile commands
config.status: executing utils/Makefile commands
config.status: executing projects/Makefile commands
config.status: executing bindings/Makefile commands
config.status: executing bindings/ocaml/Makefile.ocaml commands
=== configuring in projects/sample (/C/Users/Adam/Documents/GitHub/rust/llvm/i686-pc-mingw32/projects/sample)
configure: running /bin/sh /C/Users/Adam/Documents/GitHub/rust/src/llvm/projects/sample/configure --prefix=/usr/local  '--enable-targets=x86,x86_64' '--enable-optimized' '--disable-docs' '--enable-bindings=none' '--disable-threads' '--disable-pthreads' '--build=i686-pc-mingw32' '--host=i686-pc-mingw32' '--target=i686-p
c-mingw32' 'build_alias=i686-pc-mingw32' 'host_alias=i686-pc-mingw32' 'target_alias=i686-pc-mingw32' 'CC=gcc -m32' 'CFLAGS=-m32' 'LDFLAGS=-m32' 'CXX=g++ -m32' 'CXXFLAGS=-m32' --cache-file=/dev/null --srcdir=/C/Users/Adam/Documents/GitHub/rust/src/llvm/projects/sample
checking llvm-config... no
checking LLVM package version... unknown
checking for i686-pc-mingw32-clang... gcc -m32
checking for C compiler default output file name... a.exe
checking whether the C compiler works... yes
checking whether we are cross compiling... no
checking for suffix of executables... .exe
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc -m32 accepts -g... yes
checking for gcc -m32 option to accept ISO C89... none needed
checking whether we are using the GNU C++ compiler... yes
checking whether g++ -m32 accepts -g... yes
checking how to run the C preprocessor... gcc -m32 -E
checking build system type... i686-pc-mingw32
checking host system type... i686-pc-mingw32
checking target system type... i686-pc-mingw32
checking type of operating system we're going to host on... MingW
checking type of operating system we're going to target... MingW
checking target architecture... x86
checking for grep that handles long lines and -e... /bin/grep
checking for egrep... /bin/grep -E
checking for ANSI C header files... yes
checking for sys/types.h... yes
checking for sys/stat.h... yes
checking for stdlib.h... yes
checking for string.h... yes
checking for memory.h... yes
checking for strings.h... yes
checking for inttypes.h... yes
checking for stdint.h... yes
checking for unistd.h... yes
checking whether byte ordering is bigendian... no
checking optimization flags... -O2
checking for BSD-compatible nm... /mingw/bin/nm
checking for GNU make... make
checking whether ln -s works... no, using cp -p
checking for cmp... /bin/cmp
checking for cp... /bin/cp
checking for date... /bin/date
checking for find... /bin/find
checking for grep... (cached) /bin/grep
checking for mkdir... /bin/mkdir
checking for mv... /bin/mv
checking for i686-pc-mingw32-ranlib... no
checking for ranlib... ranlib
checking for i686-pc-mingw32-ar... no
checking for ar... ar
checking for rm... /bin/rm
checking for sed... /bin/sed
checking for tar... /bin/tar
checking for pwd... /bin/pwd
checking for Graphviz... echo Graphviz
checking for dot... echo dot
checking for fdp... echo fdp
checking for neato... echo neato
checking for twopi... echo twopi
checking for circo... echo circo
checking for gv... no
checking for gsview32... no
checking for dotty... echo dotty
checking for xdot.py... echo xdot.py
checking for a BSD-compatible install... /bin/install -c
checking for bzip2... /bin/bzip2
checking for cat... /bin/cat
checking for doxygen... no
checking for groff... no
checking for gzip... /bin/gzip
checking for pod2html... /bin/pod2html
checking for pod2man... /bin/pod2man
checking for pdfroff... no
checking for runtest... no
checking for the tclsh program in tclinclude directory... none
checking for tclsh8.4... no
checking for tclsh8.4.8... no
checking for tclsh8.4.7... no
checking for tclsh8.4.6... no
checking for tclsh8.4.5... no
checking for tclsh8.4.4... no
checking for tclsh8.4.3... no
checking for tclsh8.4.2... no
checking for tclsh8.4.1... no
checking for tclsh8.4.0... no
checking for tclsh8.3... no
checking for tclsh8.3.5... no
checking for tclsh8.3.4... no
checking for tclsh8.3.3... no
checking for tclsh8.3.2... no
checking for tclsh8.3.1... no
checking for tclsh8.3.0... no
checking for tclsh... /c/Users/Adam/AppData/Local/GitHub/PortableGit_656cc1ef6d04f06ddf8b2f5bedbac921caed3b62/bin//tclsh
checking for zip... no
checking for ocamlc... no
checking for ocamlopt... no
checking for ocamldep... no
checking for ocamldoc... no
checking for gas... no
checking for as... /mingw/bin/as
checking for linker version... 2.22
checking for compiler -Wl,-R<path> option... yes
checking for compiler -Wl,-export-dynamic option... yes
checking for compiler -Wl,--version-script option... yes
checking for an ANSI C-conforming const... yes
checking for dirent.h that defines DIR... yes
checking for library containing opendir... none required
checking dlfcn.h usability... no
checking dlfcn.h presence... no
checking for dlfcn.h... no
checking dynamic linker characteristics... Win32 ld.exe
checking which extension is used for loadable modules... .dll
checking which variable specifies run-time library path... PATH
checking for the default library search path... /lib /usr/lib
checking for objdir... .libs
checking command to parse /mingw/bin/nm output from  object... ok
checking whether libtool supports -dlopen/-dlpreopen... yes
checking for shl_load... no
checking for shl_load in -ldld... no
checking for dlopen in -ldl... no
checking for dlopen in -lsvld... no
checking for dld_link in -ldld... no
checking for _dyld_func_lookup... no
checking for _ prefix in compiled symbols... yes
checking whether deplibs are loaded by dlopen... unknown
checking argz.h usability... no
checking argz.h presence... no
checking for argz.h... no
checking for error_t... no
checking for argz_append... no
checking for argz_create_sep... no
checking for argz_insert... no
checking for argz_next... no
checking for argz_stringify... no
checking assert.h usability... yes
checking assert.h presence... yes
checking for assert.h... yes
checking ctype.h usability... yes
checking ctype.h presence... yes
checking for ctype.h... yes
checking errno.h usability... yes
checking errno.h presence... yes
checking for errno.h... yes
checking malloc.h usability... yes
checking malloc.h presence... yes
checking for malloc.h... yes
checking for memory.h... (cached) yes
checking for stdlib.h... (cached) yes
checking stdio.h usability... yes
checking stdio.h presence... yes
checking for stdio.h... yes
checking for unistd.h... (cached) yes
checking dl.h usability... no
checking dl.h presence... no
checking for dl.h... no
checking sys/dl.h usability... no
checking sys/dl.h presence... no
checking for sys/dl.h... no
checking dld.h usability... no
checking dld.h presence... no
checking for dld.h... no
checking mach-o/dyld.h usability... no
checking mach-o/dyld.h presence... no
checking for mach-o/dyld.h... no
checking for string.h... (cached) yes
checking for strchr... yes
checking for strrchr... yes
checking for memcpy... yes
checking for memmove... yes
checking for strcmp... yes
checking for closedir... yes
checking for opendir... yes
checking for readdir... yes
checking tool compatibility... ok
checking optional compiler flags... -Wno-variadic-macros -Wno-missing-field-initializers
checking for sin in -lm... yes
checking for main in -limagehlp... yes
checking for main in -lpsapi... yes
checking for library containing dlopen... no
configure: WARNING: dlopen() not found - disabling plugin support
checking for library containing mallinfo... no
checking for dirent.h that defines DIR... (cached) yes
checking for library containing opendir... (cached) none required
checking for MAP_ANONYMOUS vs. MAP_ANON... no
checking whether stat file-mode macros are broken... no
checking for sys/wait.h that is POSIX.1 compatible... no
checking whether time.h and sys/time.h may both be included... yes
checking for dlfcn.h... (cached) no
checking execinfo.h usability... no
checking execinfo.h presence... no
checking for execinfo.h... no
checking fcntl.h usability... yes
checking fcntl.h presence... yes
checking for fcntl.h... yes
checking for inttypes.h... (cached) yes
checking limits.h usability... yes
checking limits.h presence... yes
checking for limits.h... yes
checking link.h usability... no
checking link.h presence... no
checking for link.h... no
checking for malloc.h... (cached) yes
checking setjmp.h usability... yes
checking setjmp.h presence... yes
checking for setjmp.h... yes
checking signal.h usability... yes
checking signal.h presence... yes
checking for signal.h... yes
checking for stdint.h... (cached) yes
checking termios.h usability... no
checking termios.h presence... no
checking for termios.h... no
checking for unistd.h... (cached) yes
checking utime.h usability... yes
checking utime.h presence... yes
checking for utime.h... yes
checking windows.h usability... yes
checking windows.h presence... yes
checking for windows.h... yes
checking sys/mman.h usability... no
checking sys/mman.h presence... no
checking for sys/mman.h... no
checking sys/param.h usability... yes
checking sys/param.h presence... yes
checking for sys/param.h... yes
checking sys/resource.h usability... no
checking sys/resource.h presence... no
checking for sys/resource.h... no
checking sys/time.h usability... yes
checking sys/time.h presence... yes
checking for sys/time.h... yes
checking sys/uio.h usability... no
checking sys/uio.h presence... no
checking for sys/uio.h... no
checking for sys/types.h... (cached) yes
checking sys/ioctl.h usability... no
checking sys/ioctl.h presence... no
checking for sys/ioctl.h... no
checking malloc/malloc.h usability... no
checking malloc/malloc.h presence... no
checking for malloc/malloc.h... no
checking mach/mach.h usability... no
checking mach/mach.h presence... no
checking for mach/mach.h... no
checking valgrind/valgrind.h usability... no
checking valgrind/valgrind.h presence... no
checking for valgrind/valgrind.h... no
checking fenv.h usability... yes
checking fenv.h presence... yes
checking for fenv.h... yes
checking CrashReporterClient.h usability... no
checking CrashReporterClient.h presence... no
checking for CrashReporterClient.h... no
checking __crashreporter_info__... no
checking for HUGE_VAL sanity... yes
checking for pid_t... yes
checking for size_t... yes
checking whether struct tm is in sys/time.h or time.h... time.h
checking for int64_t... yes
checking for uint64_t... yes
checking for backtrace... no
checking for ceilf... yes
checking for floorf... yes
checking for roundf... yes
checking for rintf... yes
checking for nearbyintf... yes
checking for getcwd... yes
checking for powf... yes
checking for fmodf... yes
checking for strtof... yes
checking for round... yes
checking for getpagesize... yes
checking for getrusage... no
checking for getrlimit... no
checking for setrlimit... no
checking for gettimeofday... yes
checking for isatty... yes
checking for mkdtemp... no
checking for mkstemp... no
checking for mktemp... yes
checking for posix_spawn... no
checking for realpath... no
checking for sbrk... no
checking for setrlimit... (cached) no
checking for strdup... yes
checking for strerror... yes
checking for strerror_r... no
checking for setenv... no
checking for strtoll... yes
checking for strtoq... no
checking for sysconf... no
checking for malloc_zone_statistics... no
checking for setjmp... no
checking for longjmp... yes
checking for sigsetjmp... no
checking for siglongjmp... no
checking for writev... no
checking if printf has the %a format character...
checking for srand48/lrand48/drand48 in <stdlib.h>... no
checking whether strerror_s is declared... no
checking for _alloca in -lgcc... yes
checking for __alloca in -lgcc... no
checking for __chkstk in -lgcc... yes
checking for ___chkstk in -lgcc... no
checking for __ashldi3 in -lgcc... yes
checking for __ashrdi3 in -lgcc... yes
checking for __divdi3 in -lgcc... yes
checking for __fixdfdi in -lgcc... yes
checking for __fixsfdi in -lgcc... yes
checking for __floatdidf in -lgcc... yes
checking for __lshrdi3 in -lgcc... yes
checking for __moddi3 in -lgcc... yes
checking for __udivdi3 in -lgcc... yes
checking for __umoddi3 in -lgcc... yes
checking for __main in -lgcc... yes
checking for __cmpdi2 in -lgcc... yes
checking whether EnumerateLoadedModules() accepts new decl... no
checking for isnan in <math.h>... yes
checking for isnan in <cmath>... no
checking for std::isnan in <cmath>... yes
checking for isinf in <math.h>... yes
checking for isinf in <cmath>... no
checking for std::isinf in <cmath>... yes
checking for finite in <ieeefp.h>... no
checking for GCC atomic builtins... no
configure: WARNING: LLVM will be built thread-unsafe because atomic builtins are missing
checking for __dso_handle... no
checking for compiler -fvisibility-inlines-hidden option... yes
configure: creating ./config.status
config.status: creating Makefile.common
config.status: creating Makefile.llvm.config
config.status: executing setup commands
config.status: executing Makefile commands
config.status: executing lib/Makefile commands
config.status: executing lib/sample/Makefile commands
config.status: executing tools/Makefile commands
config.status: executing tools/sample/Makefile commands
configure:
configure: writing configuration
configure:
configure: CFG_SRC_DIR          := /C/Users/Adam/Documents/GitHub/rust ...
configure: CFG_BUILD_DIR        := /C/Users/Adam/Documents/GitHub/rust ...
configure: CFG_OSTYPE           := pc-mingw32
configure: CFG_CPUTYPE          := i686
configure: CFG_CONFIGURE_ARGS   :=
configure: CFG_PREFIX           := /usr/local
configure: CFG_TARGET_TRIPLES   := i686-pc-mingw32
configure: CFG_C_COMPILER       := gcc
configure: CFG_LIBDIR           := bin
configure: CFG_DISABLE_MANAGE_SUBMODULES :=
configure: CFG_BAD_VALGRIND     := 1
configure: CFG_LLVM_ROOT        :=
configure: CFG_LLVM_SRC_DIR     := /C/Users/Adam/Documents/GitHub/rust ...
configure: CFG_LLVM_BUILD_DIR_i686_pc_mingw32 := /C/Users/Adam/Documents/GitHub/rust ...
configure: CFG_LLVM_INST_DIR_i686_pc_mingw32 := /C/Users/Adam/Documents/GitHub/rust ...
configure:
configure: leaving ./Makefile unchanged
configure: mv config.tmp config.mk
configure:
configure: complete
configure:

Adam@WEEDENASUS1 /C/Users/Adam/Documents/GitHub/rust
$ make
cfg: shell host triple i686-pc-mingw32
Makefile:108: /C/Users/Adam/Documents/GitHub/rust/mk/platform.mk: No such file or directory
cfg: disabling valgrind due to its unreliability on this platform
Makefile:487: /C/Users/Adam/Documents/GitHub/rust/mk/target.mk: No such file or directory
Makefile:488: /C/Users/Adam/Documents/GitHub/rust/mk/host.mk: No such file or directory
Makefile:489: /C/Users/Adam/Documents/GitHub/rust/mk/stage0.mk: No such file or directory
Makefile:490: /C/Users/Adam/Documents/GitHub/rust/mk/rt.mk: No such file or directory
Makefile:491: /C/Users/Adam/Documents/GitHub/rust/mk/rustllvm.mk: No such file or directory
Makefile:492: /C/Users/Adam/Documents/GitHub/rust/mk/tools.mk: No such file or directory
Makefile:493: /C/Users/Adam/Documents/GitHub/rust/mk/docs.mk: No such file or directory
Makefile:494: /C/Users/Adam/Documents/GitHub/rust/mk/llvm.mk: No such file or directory
make: *** No rule to make target `/C/Users/Adam/Documents/GitHub/rust/mk/llvm.mk'.  Stop.
