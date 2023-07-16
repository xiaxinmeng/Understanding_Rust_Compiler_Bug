
configure: looking for configure programs
configure: found cmp
configure: found mkdir
configure: found printf
configure: found cut
configure: found head
configure: found grep
configure: found xargs
configure: found cp
configure: found find
configure: found uname
configure: found date
configure: found tr
configure: found sed
configure: found file
configure: inspecting environment
configure: recreating config.tmp
configure: 
configure: processing /mnt/disk2/opt/pkgs/rust/configure args
configure: 
configure: CFG_PREFIX           := /usr/local 
configure: CFG_LOCAL_RUST_ROOT  := /usr/local 
configure: CFG_LLVM_ROOT        :=  
configure: CFG_ANDROID_CROSS_PATH := /opt/ndk_standalone 
configure: CFG_MINGW32_CROSS_PATH :=  
configure: CFG_BUILD            := x86_64-unknown-linux-gnu 
configure: CFG_HOST             := x86_64-unknown-linux-gnu 
configure: CFG_TARGET           := x86_64-unknown-linux-gnu 
configure: CFG_LOCALSTATEDIR    := /var/lib 
configure: CFG_SYSCONFDIR       := /etc 
configure: CFG_DATADIR          := /usr/local/share 
configure: CFG_INFODIR          := /usr/local/share/info 
configure: CFG_MANDIR           := /usr/local/share/man 
configure: CFG_LIBDIR           := /usr/local/lib 
configure: CFG_RUSTLIBDIR       := rustlib 
configure: 
configure: validating /mnt/disk2/opt/pkgs/rust/configure args
configure: 
configure: 
configure: looking for build programs
configure: 
configure: CFG_PERL             := /usr/bin/perl 
configure: CFG_CURLORWGET       := /usr/bin/curl (7.32.0)
configure: CFG_PYTHON           := /usr/bin/python2.7 
configure: CFG_GIT              := /usr/bin/git (1.8.3.2)
configure: CFG_CLANG            :=  
configure: CFG_CCACHE           :=  
configure: CFG_GCC              := /usr/bin/gcc (4.8.1-10ubuntu9))
configure: CFG_LD               := /usr/bin/ld (2.23.52.20130913)
configure: CFG_VALGRIND         := /usr/bin/valgrind (3.8.1)
configure: CFG_PERF             :=  
configure: CFG_ISCC             :=  
configure: CFG_LLNEXTGEN        :=  
configure: CFG_PANDOC           :=  
configure: CFG_PDFLATEX         := /usr/bin/pdflatex (3.1415926-2.5-1.40.14)
configure: CFG_XETEX            := /usr/bin/xetex (3.1415926-2.5-0.9999.3-2013103115)
configure: CFG_LUATEX           := /usr/bin/luatex (0.76.0-2013070111)
configure: CFG_NODE             :=  
configure: CFG_GDB              := /usr/bin/gdb (7.6.1-ubuntu)
configure: CFG_PAXCTL           :=  
configure: CFG_ZCAT             := /bin/zcat (1.6)
configure: 
configure: looking for target specific programs
configure: 
configure: CFG_ADB              :=  
configure: 
configure: GRSecurity: no
configure: 
configure: 
configure: making directories
configure: 
configure: 
configure: configuring submodules
configure: 
configure: git: submodule sync
Synchronizing submodule url for 'src/gyp'
Synchronizing submodule url for 'src/libuv'
Synchronizing submodule url for 'src/llvm'
configure: git: submodule update
configure: git: submodule foreach sync
Entering 'src/gyp'
Entering 'src/libuv'
Entering 'src/llvm'
configure: git: submodule foreach update
configure: git: submodule status
 1e46da1000bc29679ab4cebf3c1034cb7d6f4487 src/gyp (heads/master)
 fd5308383c575472edb2163d823dc6670bf59609 src/libuv (v0.11.7-249-gfd53083)
 535989a92ce1f6f6488c94a2c8f4ed438349f162 src/llvm (remotes/origin/rust-llvm-2014-01-22)
configure: git: submodule clobber
Entering 'src/gyp'
Removing pylib/gyp/__init__.pyc
Removing pylib/gyp/common.pyc
Removing pylib/gyp/generator/__init__.pyc
Removing pylib/gyp/generator/make.pyc
Removing pylib/gyp/input.pyc
Removing pylib/gyp/xcode_emulation.pyc
Entering 'src/libuv'
Entering 'src/llvm'
Entering 'src/gyp'
Entering 'src/libuv'
Entering 'src/llvm'
configure: 
configure: looking at LLVM
configure: 
configure: configuring LLVM for x86_64-unknown-linux-gnu
configure: configuring LLVM with:
configure: --enable-targets=x86,x86_64,arm,mips --enable-optimized --enable-assertions --disable-docs --enable-bindings=none --disable-terminfo --disable-zlib --disable-libffi --build=x86_64-unknown-linux-gnu                         --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --with-python=/usr/bin/python2.7
checking for x86_64-unknown-linux-gnu-clang... gcc
checking for C compiler default output file name... a.out
checking whether the C compiler works... yes
checking whether we are cross compiling... no
checking for suffix of executables... 
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc accepts -g... yes
checking for gcc option to accept ISO C89... none needed
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking how to run the C preprocessor... gcc -E
checking build system type... x86_64-unknown-linux-gnu
checking host system type... x86_64-unknown-linux-gnu
checking target system type... x86_64-unknown-linux-gnu
checking type of operating system we're going to host on... Linux
checking type of operating system we're going to target... Linux
checking target architecture... x86_64
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
checking optimization flags... -O3
checking for BSD-compatible nm... /usr/bin/nm -B
checking for GNU make... make
checking whether ln -s works... yes
checking for cmp... /usr/bin/cmp
checking for cp... /bin/cp
checking for date... /bin/date
checking for find... /usr/bin/find
checking for grep... (cached) /bin/grep
checking for mkdir... /bin/mkdir
checking for mv... /bin/mv
checking for x86_64-unknown-linux-gnu-ranlib... no
checking for ranlib... ranlib
checking for x86_64-unknown-linux-gnu-ar... no
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
checking for gv... /usr/bin/gv
checking for dotty... echo dotty
checking for xdot... no
checking for xdot.py... no
checking for a BSD-compatible install... /usr/bin/install -c
checking for bzip2... /bin/bzip2
checking for cat... /bin/cat
checking for doxygen... /usr/bin/doxygen
checking for groff... /usr/bin/groff
checking for gzip... /bin/gzip
checking for pdfroff... /usr/bin/pdfroff
checking for zip... /usr/bin/zip
checking for ocamlc... no
checking for ocamlopt... no
checking for ocamldep... no
checking for ocamldoc... no
checking for gas... no
checking for as... /usr/bin/as
checking for linker version... 2.23.52.20130913
checking for compiler -Wl,-R<path> option... yes
checking for compiler -rdynamic option... yes
checking for compiler -Wl,--version-script option... yes
checking for an ANSI C-conforming const... yes
checking for dirent.h that defines DIR... yes
checking for library containing opendir... none required
checking dlfcn.h usability... yes
checking dlfcn.h presence... yes
checking for dlfcn.h... yes
checking dynamic linker characteristics... GNU/Linux ld.so
checking which extension is used for loadable modules... .so
checking for the default library search path... /lib /usr/lib /usr/local/lib /usr/lib/nvidia-settings-319-updates /lib/x86_64-linux-gnu /usr/lib/x86_64-linux-gnu /usr/lib/x86_64-linux-gnu/mesa-egl /usr/lib/nvidia-319-updates /usr/lib32/nvidia-319-updates 
checking for objdir... .libs
checking command to parse /usr/bin/nm -B output from  object... ok
checking whether libtool supports -dlopen/-dlpreopen... yes
checking for shl_load... no
checking for shl_load in -ldld... no
checking for dlopen in -ldl... yes
checking for dlerror... yes
checking for _ prefix in compiled symbols... no
checking whether deplibs are loaded by dlopen... yes
checking argz.h usability... yes
checking argz.h presence... yes
checking for argz.h... yes
checking for error_t... yes
checking for argz_append... yes
checking for argz_create_sep... yes
checking for argz_insert... yes
checking for argz_next... yes
checking for argz_stringify... yes
checking errno.h usability... yes
checking errno.h presence... yes
checking for errno.h... yes
checking malloc.h usability... yes
checking malloc.h presence... yes
checking for malloc.h... yes
checking for memory.h... (cached) yes
checking for unistd.h... (cached) yes
checking mach-o/dyld.h usability... no
checking mach-o/dyld.h presence... no
checking for mach-o/dyld.h... no
checking for closedir... yes
checking for opendir... yes
checking for readdir... yes
checking tool compatibility... ok
checking optional compiler flags... -Wno-variadic-macros -Wno-missing-field-initializers   -Wno-maybe-uninitialized
checking for python... user defined: /usr/bin/python2.7
checking for python >= 2.5... /usr/bin/python2.7 (2.7.5+)
checking for sin in -lm... yes
checking for library containing dlopen... -ldl
checking for library containing clock_gettime... none required
checking for library containing mallinfo... none required
checking for pthread_mutex_init in -lpthread... yes
checking for library containing pthread_mutex_lock... none required
checking for library containing pthread_rwlock_init... none required
checking for library containing pthread_getspecific... none required
checking for xml2-config... no
checking for libxml2 includes... xml2-config not found
checking for dirent.h that defines DIR... (cached) yes
checking for library containing opendir... (cached) none required
checking for MAP_ANONYMOUS vs. MAP_ANON... yes
checking whether stat file-mode macros are broken... no
checking for sys/wait.h that is POSIX.1 compatible... yes
checking whether time.h and sys/time.h may both be included... yes
checking how to run the C++ preprocessor... g++ -E
checking cxxabi.h usability... yes
checking cxxabi.h presence... yes
checking for cxxabi.h... yes
checking for dlfcn.h... (cached) yes
checking execinfo.h usability... yes
checking execinfo.h presence... yes
checking for execinfo.h... yes
checking fcntl.h usability... yes
checking fcntl.h presence... yes
checking for fcntl.h... yes
checking for inttypes.h... (cached) yes
checking link.h usability... yes
checking link.h presence... yes
checking for link.h... yes
checking for malloc.h... (cached) yes
checking setjmp.h usability... yes
checking setjmp.h presence... yes
checking for setjmp.h... yes
checking signal.h usability... yes
checking signal.h presence... yes
checking for signal.h... yes
checking for stdint.h... (cached) yes
checking termios.h usability... yes
checking termios.h presence... yes
checking for termios.h... yes
checking for unistd.h... (cached) yes
checking utime.h usability... yes
checking utime.h presence... yes
checking for utime.h... yes
checking sys/mman.h usability... yes
checking sys/mman.h presence... yes
checking for sys/mman.h... yes
checking sys/param.h usability... yes
checking sys/param.h presence... yes
checking for sys/param.h... yes
checking sys/resource.h usability... yes
checking sys/resource.h presence... yes
checking for sys/resource.h... yes
checking sys/time.h usability... yes
checking sys/time.h presence... yes
checking for sys/time.h... yes
checking sys/uio.h usability... yes
checking sys/uio.h presence... yes
checking for sys/uio.h... yes
checking sys/ioctl.h usability... yes
checking sys/ioctl.h presence... yes
checking for sys/ioctl.h... yes
checking malloc/malloc.h usability... no
checking malloc/malloc.h presence... no
checking for malloc/malloc.h... no
checking mach/mach.h usability... no
checking mach/mach.h presence... no
checking for mach/mach.h... no
checking valgrind/valgrind.h usability... yes
checking valgrind/valgrind.h presence... yes
checking for valgrind/valgrind.h... yes
checking fenv.h usability... yes
checking fenv.h presence... yes
checking for fenv.h... yes
checking whether FE_ALL_EXCEPT is declared... yes
checking whether FE_INEXACT is declared... yes
checking pthread.h usability... yes
checking pthread.h presence... yes
checking for pthread.h... yes
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
checking for backtrace... yes
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
checking for log... yes
checking for log2... yes
checking for log10... yes
checking for exp... yes
checking for exp2... yes
checking for getpagesize... yes
checking for getrusage... yes
checking for getrlimit... yes
checking for setrlimit... yes
checking for gettimeofday... yes
checking for isatty... yes
checking for mkdtemp... yes
checking for mkstemp... yes
checking for mktemp... yes
checking for posix_spawn... yes
checking for pread... yes
checking for realpath... yes
checking for sbrk... yes
checking for setrlimit... (cached) yes
checking for strerror... yes
checking for strerror_r... yes
checking for setenv... yes
checking for arc4random... no
checking for strtoll... yes
checking for strtoq... yes
checking for sysconf... yes
checking for malloc_zone_statistics... no
checking for setjmp... yes
checking for longjmp... yes
checking for sigsetjmp... no
checking for siglongjmp... yes
checking for writev... yes
checking for futimes... yes
checking for futimens... yes
checking if printf has the %a format character... yes
checking for srand48/lrand48/drand48 in <stdlib.h>... yes
checking whether strerror_s is declared... no
checking for isnan in <math.h>... yes
checking for isnan in <cmath>... yes
checking for std::isnan in <cmath>... yes
checking for isinf in <math.h>... yes
checking for isinf in <cmath>... yes
checking for std::isinf in <cmath>... yes
checking for finite in <ieeefp.h>... no
checking for stdlib.h... (cached) yes
checking for unistd.h... (cached) yes
checking for getpagesize... (cached) yes
checking for working mmap... yes
checking for mmap of files... yes
checking if /dev/zero is needed for mmap... no
checking for GCC atomic builtins... yes
checking for 32-bit userspace on 64-bit system... no
checking for __dso_handle... yes
checking for compiler -fvisibility-inlines-hidden option... yes
configure: creating ./config.status
config.status: creating include/llvm/Config/Targets.def
config.status: creating include/llvm/Config/AsmPrinters.def
config.status: creating include/llvm/Config/AsmParsers.def
config.status: creating include/llvm/Config/Disassemblers.def
config.status: creating Makefile.config
config.status: creating llvm.spec
config.status: creating docs/doxygen.cfg
config.status: creating bindings/ocaml/llvm/META.llvm
config.status: creating include/llvm/Config/config.h
config.status: creating include/llvm/Config/llvm-config.h
config.status: creating include/llvm/Support/DataTypes.h
config.status: include/llvm/Support/DataTypes.h is unchanged
config.status: executing setup commands
config.status: executing Makefile commands
config.status: executing Makefile.common commands
config.status: executing examples/Makefile commands
config.status: executing lib/Makefile commands
config.status: executing test/Makefile commands
config.status: executing test/Makefile.tests commands
config.status: executing unittests/Makefile commands
config.status: executing tools/Makefile commands
config.status: executing utils/Makefile commands
config.status: executing projects/Makefile commands
config.status: executing bindings/Makefile commands
config.status: executing bindings/ocaml/Makefile.ocaml commands
=== configuring in projects/sample (/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/llvm/projects/sample)
configure: running /bin/bash /mnt/disk2/opt/pkgs/rust/src/llvm/projects/sample/configure --prefix=/usr/local  '--enable-targets=x86,x86_64,arm,mips' '--enable-optimized' '--enable-assertions' '--disable-docs' '--enable-bindings=none' '--disable-terminfo' '--disable-zlib' '--disable-libffi' '--build=x86_64-unknown-linux-gnu' '--host=x86_64-unknown-linux-gnu' '--target=x86_64-unknown-linux-gnu' '--with-python=/usr/bin/python2.7' 'build_alias=x86_64-unknown-linux-gnu' 'host_alias=x86_64-unknown-linux-gnu' 'target_alias=x86_64-unknown-linux-gnu' 'CC=gcc' 'CFLAGS=' 'LDFLAGS=' 'CXX=g++' 'CXXFLAGS=' --cache-file=/dev/null --srcdir=/mnt/disk2/opt/pkgs/rust/src/llvm/projects/sample
checking llvm-config... /mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llvm-config
checking LLVM package version... 3.5svn
checking for x86_64-unknown-linux-gnu-clang... gcc
checking for C compiler default output file name... a.out
checking whether the C compiler works... yes
checking whether we are cross compiling... no
checking for suffix of executables... 
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc accepts -g... yes
checking for gcc option to accept ISO C89... none needed
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking how to run the C preprocessor... gcc -E
checking build system type... x86_64-unknown-linux-gnu
checking host system type... x86_64-unknown-linux-gnu
checking target system type... x86_64-unknown-linux-gnu
checking type of operating system we're going to host on... Linux
checking type of operating system we're going to target... Linux
checking target architecture... x86_64
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
checking optimization flags... -O3
checking for BSD-compatible nm... /usr/bin/nm -B
checking for GNU make... make
checking whether ln -s works... yes
checking for cmp... /usr/bin/cmp
checking for cp... /bin/cp
checking for date... /bin/date
checking for find... /usr/bin/find
checking for grep... (cached) /bin/grep
checking for mkdir... /bin/mkdir
checking for mv... /bin/mv
checking for x86_64-unknown-linux-gnu-ranlib... no
checking for ranlib... ranlib
checking for x86_64-unknown-linux-gnu-ar... no
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
checking for gv... /usr/bin/gv
checking for dotty... echo dotty
checking for xdot.py... echo xdot.py
checking for a BSD-compatible install... /usr/bin/install -c
checking for bzip2... /bin/bzip2
checking for cat... /bin/cat
checking for doxygen... /usr/bin/doxygen
checking for groff... /usr/bin/groff
checking for gzip... /bin/gzip
checking for pod2html... /usr/bin/pod2html
checking for pod2man... /usr/bin/pod2man
checking for pdfroff... /usr/bin/pdfroff
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
checking for tclsh... /usr/bin/tclsh
checking for zip... /usr/bin/zip
checking for ocamlc... no
checking for ocamlopt... no
checking for ocamldep... no
checking for ocamldoc... no
checking for gas... no
checking for as... /usr/bin/as
checking for linker version... 2.23.52.20130913
checking for compiler -Wl,-R<path> option... yes
checking for compiler -rdynamic option... yes
checking for compiler -Wl,--version-script option... yes
checking for an ANSI C-conforming const... yes
checking for dirent.h that defines DIR... yes
checking for library containing opendir... none required
checking dlfcn.h usability... yes
checking dlfcn.h presence... yes
checking for dlfcn.h... yes
checking dynamic linker characteristics... GNU/Linux ld.so
checking which extension is used for loadable modules... .so
checking for the default library search path... /lib /usr/lib /usr/local/lib /usr/lib/nvidia-settings-319-updates /lib/x86_64-linux-gnu /usr/lib/x86_64-linux-gnu /usr/lib/x86_64-linux-gnu/mesa-egl /usr/lib/nvidia-319-updates /usr/lib32/nvidia-319-updates 
checking for objdir... .libs
checking command to parse /usr/bin/nm -B output from  object... ok
checking whether libtool supports -dlopen/-dlpreopen... yes
checking for shl_load... no
checking for shl_load in -ldld... no
checking for dlopen in -ldl... yes
checking for dlerror... yes
checking for _ prefix in compiled symbols... no
checking whether deplibs are loaded by dlopen... yes
checking argz.h usability... yes
checking argz.h presence... yes
checking for argz.h... yes
checking for error_t... yes
checking for argz_append... yes
checking for argz_create_sep... yes
checking for argz_insert... yes
checking for argz_next... yes
checking for argz_stringify... yes
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
checking for library containing dlopen... -ldl
checking for library containing clock_gettime... none required
checking for library containing mallinfo... none required
checking for pthread_mutex_init in -lpthread... yes
checking for library containing pthread_mutex_lock... none required
checking for library containing pthread_rwlock_init... none required
checking for library containing pthread_getspecific... none required
checking for dirent.h that defines DIR... (cached) yes
checking for library containing opendir... (cached) none required
checking for MAP_ANONYMOUS vs. MAP_ANON... yes
checking whether stat file-mode macros are broken... no
checking for sys/wait.h that is POSIX.1 compatible... yes
checking whether time.h and sys/time.h may both be included... yes
checking for dlfcn.h... (cached) yes
checking execinfo.h usability... yes
checking execinfo.h presence... yes
checking for execinfo.h... yes
checking fcntl.h usability... yes
checking fcntl.h presence... yes
checking for fcntl.h... yes
checking for inttypes.h... (cached) yes
checking limits.h usability... yes
checking limits.h presence... yes
checking for limits.h... yes
checking link.h usability... yes
checking link.h presence... yes
checking for link.h... yes
checking for malloc.h... (cached) yes
checking setjmp.h usability... yes
checking setjmp.h presence... yes
checking for setjmp.h... yes
checking signal.h usability... yes
checking signal.h presence... yes
checking for signal.h... yes
checking for stdint.h... (cached) yes
checking termios.h usability... yes
checking termios.h presence... yes
checking for termios.h... yes
checking for unistd.h... (cached) yes
checking utime.h usability... yes
checking utime.h presence... yes
checking for utime.h... yes
checking windows.h usability... no
checking windows.h presence... no
checking for windows.h... no
checking sys/mman.h usability... yes
checking sys/mman.h presence... yes
checking for sys/mman.h... yes
checking sys/param.h usability... yes
checking sys/param.h presence... yes
checking for sys/param.h... yes
checking sys/resource.h usability... yes
checking sys/resource.h presence... yes
checking for sys/resource.h... yes
checking sys/time.h usability... yes
checking sys/time.h presence... yes
checking for sys/time.h... yes
checking sys/uio.h usability... yes
checking sys/uio.h presence... yes
checking for sys/uio.h... yes
checking for sys/types.h... (cached) yes
checking sys/ioctl.h usability... yes
checking sys/ioctl.h presence... yes
checking for sys/ioctl.h... yes
checking malloc/malloc.h usability... no
checking malloc/malloc.h presence... no
checking for malloc/malloc.h... no
checking mach/mach.h usability... no
checking mach/mach.h presence... no
checking for mach/mach.h... no
checking valgrind/valgrind.h usability... yes
checking valgrind/valgrind.h presence... yes
checking for valgrind/valgrind.h... yes
checking fenv.h usability... yes
checking fenv.h presence... yes
checking for fenv.h... yes
checking pthread.h usability... yes
checking pthread.h presence... yes
checking for pthread.h... yes
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
checking for backtrace... yes
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
checking for getrusage... yes
checking for getrlimit... yes
checking for setrlimit... yes
checking for gettimeofday... yes
checking for isatty... yes
checking for mkdtemp... yes
checking for mkstemp... yes
checking for mktemp... yes
checking for posix_spawn... yes
checking for realpath... yes
checking for sbrk... yes
checking for setrlimit... (cached) yes
checking for strdup... yes
checking for strerror... yes
checking for strerror_r... yes
checking for setenv... yes
checking for strtoll... yes
checking for strtoq... yes
checking for sysconf... yes
checking for malloc_zone_statistics... no
checking for setjmp... yes
checking for longjmp... yes
checking for sigsetjmp... no
checking for siglongjmp... yes
checking for writev... yes
checking if printf has the %a format character... yes
checking for srand48/lrand48/drand48 in <stdlib.h>... yes
checking whether strerror_s is declared... no
checking for isnan in <math.h>... yes
checking for isnan in <cmath>... yes
checking for std::isnan in <cmath>... yes
checking for isinf in <math.h>... yes
checking for isinf in <cmath>... yes
checking for std::isinf in <cmath>... yes
checking for finite in <ieeefp.h>... no
checking for stdlib.h... (cached) yes
checking for unistd.h... (cached) yes
checking for getpagesize... (cached) yes
checking for working mmap... yes
checking for mmap of files... yes
checking if /dev/zero is needed for mmap... no
checking for GCC atomic builtins... yes
checking for 32-bit userspace on 64-bit system... no
checking for __dso_handle... yes
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
configure: CFG_SRC_DIR          := /mnt/disk2/opt/pkgs/rust/ 
configure: CFG_BUILD_DIR        := /mnt/disk2/opt/pkgs/rust/ 
configure: CFG_OSTYPE           := unknown-linux-gnu 
configure: CFG_CPUTYPE          := x86_64 
configure: CFG_CONFIGURE_ARGS   :=  
configure: CFG_PREFIX           := /usr/local 
configure: CFG_BUILD            := x86_64-unknown-linux-gnu 
configure: CFG_HOST             := x86_64-unknown-linux-gnu 
configure: CFG_TARGET           := x86_64-unknown-linux-gnu 
configure: CFG_C_COMPILER       := gcc 
configure: CFG_LIBDIR           := /usr/local/lib 
configure: CFG_RUSTLIBDIR       := rustlib 
configure: CFG_LIBDIR_RELATIVE  := lib 
configure: CFG_DISABLE_MANAGE_SUBMODULES :=  
configure: CFG_ANDROID_CROSS_PATH := /opt/ndk_standalone 
configure: CFG_MINGW32_CROSS_PATH :=  
configure: CFG_MANDIR           := /usr/local/share/man 
configure: CFG_DISABLE_INJECT_STD_VERSION :=  
configure: CFG_LLVM_ROOT        :=  
configure: CFG_LLVM_SRC_DIR     := /mnt/disk2/opt/pkgs/rust/src/llvm/ 
configure: CFG_LLVM_BUILD_DIR_x86_64_unknown_linux_gnu := /mnt/disk2/opt/pkgs/rust/x86_64-unk ...
configure: CFG_LLVM_INST_DIR_x86_64_unknown_linux_gnu := /mnt/disk2/opt/pkgs/rust/x86_64-unk ...
configure: 
configure: leaving ./Makefile unchanged
configure: leaving config.mk unchanged
configure: 
configure: complete
configure: 
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: using gcc
cfg: no pandoc found, omitting docs
cfg: no node found, omitting docs
cfg: no llnextgen found, omitting grammar-verification
compile: x86_64-unknown-linux-gnu/rt/rust_builtin.o
compile: x86_64-unknown-linux-gnu/rt/rust_android_dummy.o
compile: x86_64-unknown-linux-gnu/rt/rust_test_helpers.o
compile: x86_64-unknown-linux-gnu/rt/rust_try.o
compile: x86_64-unknown-linux-gnu/rt/arch/x86_64/_context.o
compile: x86_64-unknown-linux-gnu/rt/arch/x86_64/record_sp.o
fetch: x86_64-unknown-linux-gnu/stage0/bin/rustc
compile: x86_64-unknown-linux-gnu/rt/arch/x86_64/morestack.o
compile: x86_64-unknown-linux-gnu/rt/miniz.o
compile: x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o
compile: x86_64-unknown-linux-gnu/rustllvm/PassWrapper.o
(cd /mnt/disk2/opt/pkgs/rust/src/libuv/ && /usr/bin/python2.7 ./gyp_uv.py -f make -Dtarget_arch=x64 -D ninja -DOS=linux -Goutput_dir=/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv --generator-output /mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv)
compile: x86_64-unknown-linux-gnu/rt/rust_uv.o
compile: x86_64-unknown-linux-gnu/rt/sundown/src/autolink.o
['-f', 'make', '-Dtarget_arch=x64', '-D', 'ninja', '-DOS=linux', '-Goutput_dir=/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv', '--generator-output', '/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv', '/mnt/disk2/opt/pkgs/rust/src/libuv/uv.gyp', '-I', '/mnt/disk2/opt/pkgs/rust/src/libuv/common.gypi', '--depth=.', '-Dgcc_version=48', '-Dclang=0', '-Dhost_arch=x64', '-Dlibrary=static_library', '-Dcomponent=static_library']
compile: x86_64-unknown-linux-gnu/rt/sundown/src/buffer.o
touch /mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv/Makefile
compile: x86_64-unknown-linux-gnu/rt/sundown/src/stack.o
compile: x86_64-unknown-linux-gnu/rt/sundown/src/markdown.o
compile: x86_64-unknown-linux-gnu/rt/sundown/html/houdini_href_e.o
compile: x86_64-unknown-linux-gnu/rt/sundown/html/houdini_html_e.o
compile: x86_64-unknown-linux-gnu/rt/sundown/html/html_smartypants.o
compile: x86_64-unknown-linux-gnu/rt/sundown/html/html.o
link: x86_64-unknown-linux-gnu/rt/librustrt.a
link: x86_64-unknown-linux-gnu/rt/libmorestack.a
link: x86_64-unknown-linux-gnu/rt/libminiz.a
link: x86_64-unknown-linux-gnu/rt/librustllvm.a
make[1]: Entering directory `/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv'
make[1]: Nothing to be done for `all'.
make[1]: Leaving directory `/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/rt/libuv'
link: x86_64-unknown-linux-gnu/rt/libuv_support.a
link: x86_64-unknown-linux-gnu/rt/libsundown.a
determined most recent snapshot: rust-stage0-2014-01-20-b6400f9-linux-x86_64-ad8b455804ff46aa721db9453438591da4c35b48.tar.bz2
got download with ok hash
opening snapshot dl/rust-stage0-2014-01-20-b6400f9-linux-x86_64-ad8b455804ff46aa721db9453438591da4c35b48.tar.bz2
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustuv-09e0b925-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.std
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate-14e9482b-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustuv
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-966edb7e-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-ebc61d75-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-0d83f02f-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgreen-80d9e76a-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libextra-64ade3d6-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.extra
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmorestack.a
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.green
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate-14e9482b-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgreen-80d9e76a-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-966edb7e-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libextra-64ade3d6-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-0d83f02f-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.flate
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustuv-09e0b925-0.10-pre.rlib
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-ebc61d75-0.10-pre.so
removing x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc
removing x86_64-unknown-linux-gnu/stage0/bin/rustc
extracting rust-stage0/bin/rustc
cp: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmorestack.a
cp: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmorestack.a
cp: x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmorestack.a
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libextra
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgreen
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustuv
cp: x86_64-unknown-linux-gnu/stage1/lib/libstd
cp: x86_64-unknown-linux-gnu/stage1/lib/libflate
cp: x86_64-unknown-linux-gnu/stage1/lib/libgreen
cp: x86_64-unknown-linux-gnu/stage1/lib/librustuv
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax
cp: x86_64-unknown-linux-gnu/stage1/lib/libextra
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc
cp: x86_64-unknown-linux-gnu/stage1/lib/libsyntax
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc
cp: x86_64-unknown-linux-gnu/stage1/lib/librustc
cp: x86_64-unknown-linux-gnu/stage1/bin/rustc
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libextra
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgreen
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustuv
cp: x86_64-unknown-linux-gnu/stage2/lib/libstd
cp: x86_64-unknown-linux-gnu/stage2/lib/libflate
cp: x86_64-unknown-linux-gnu/stage2/lib/libgreen
cp: x86_64-unknown-linux-gnu/stage2/lib/librustuv
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax
cp: x86_64-unknown-linux-gnu/stage2/lib/libextra
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc
cp: x86_64-unknown-linux-gnu/stage2/lib/libsyntax
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc
cp: x86_64-unknown-linux-gnu/stage2/lib/librustc
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustpkg
error: linking with `cc` failed: exit code: 1
note: cc arguments: '-m64' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-o' 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc' 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc.o' '-Wl,--as-needed' '-Wl,-O1' '-L/mnt/disk2/opt/pkgs/rust/.rust' '-L/mnt/disk2/opt/pkgs/rust' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lstd-966edb7e-0.10-pre' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lgreen-80d9e76a-0.10-pre' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lrustuv-09e0b925-0.10-pre' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lextra-64ade3d6-0.10-pre' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lflate-14e9482b-0.10-pre' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lsyntax-ebc61d75-0.10-pre' '-L/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-lrustc-0d83f02f-0.10-pre' '-lstdc++' '-lrt' '-lpthread' '-lrt' '-ldl' '-lm' '-lpthread' '-lmorestack' '-Wl,-rpath,$ORIGIN/../lib' '-Wl,-rpath,/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib' '-Wl,-rpath,/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib'
note: /mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-0d83f02f-0.10-pre.so: undefined reference to `LLVMRustAddModuleFlag'
/mnt/disk2/opt/pkgs/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-0d83f02f-0.10-pre.so: undefined reference to `LLVMRustDebugMetadataVersion'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc] Error 101
make: *** Waiting for unfinished jobs....
