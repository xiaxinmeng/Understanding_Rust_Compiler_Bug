plain
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/36d65d00164d1750f6fa7f8b0f52dabc3fea500b.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/eebda16e4b45f2eed4310cf7b9872cc752278163.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/jemalloc'
---
[00:00:01] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:01] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:01] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:01] Submodule 'src/jemalloc' (https://github.com/rust-lang/jemalloc.git) registered for path 'src/jemalloc'
[00:00:01] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:01] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:01] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd) registered for path 'src/stdsimd'
[00:00:01] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:01] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
---
[00:00:12] Submodule path 'src/dlmalloc': checked out 'c99638dc2ecfc750cc1656f6edb2bd062c1e0981'
[00:00:12] Submodule path 'src/doc/nomicon': checked out '748a5e6742db4a21c4c630a58087f818828e8a0a'
[00:00:13] Submodule path 'src/doc/reference': checked out '134f419ee62714590b04712fe6072253bc2a7822'
[00:00:13] Submodule path 'src/jemalloc': checked out '1f5a28755e301ac581e2048011e4e0ff3da482ef'
[00:00:13] Submodule path 'src/libbacktrace': checked out '7845b4ecc7e8a6856ea42bb0f1f6bfcdbb8926ca'
[00:00:13] Submodule 'compiler-rt' (https://github.com/rust-lang/compiler-rt) registered for path 'src/libcompiler_builtins/compiler-rt'
[00:00:13] Cloning into '/Users/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
[00:00:21] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out 'dfd960b5f1a1751b22738fa34fd27b583f4618db'
[00:00:21] Submodule path 'src/liblibc': checked out 'a7e78a78e17c8776d7780008ccb3ce541ec64ae9'
---
[00:02:44]       Memory: 8 GB
[00:02:44]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:44]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:44]       SMC Version (system): 2.8f0
[00:02:44]       Serial Number (system): VMeKjbnRg6VF
[00:02:44] 
[00:02:44] hw.ncpu: 4
[00:02:44] hw.byteorder: 1234
[00:02:44] hw.memsize: 8589934592
---
[00:03:34] [RUSTC-TIMING] alloc_jemalloc test:false 0.180
[00:03:56] error: failed to run custom build command for `std v0.0.0 (file:///Users/travis/build/rust-lang/rust/src/libstd)`
[00:03:56] process didn't exit successfully: `/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/release/build/std-843dad02bb189003/build-script-build` (exit code: 1)
[00:03:56] --- stdout
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/ztest.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/xcoff.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/unknown.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/ttest.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/testlib.h
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/testlib.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/stest.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/state.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/sort.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/simple.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/README.md
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/read.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/print.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/posix.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/pecoff.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/nounwind.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/move-if-change
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/mmapio.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/mmap.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/missing
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/Mark.Twain-Tom.Sawyer.txt
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/Makefile.in
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/Makefile.am
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/ltmain.sh
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/LICENSE
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/internal.h
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/install-sh
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/filetype.awk
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/filenames.h
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/fileline.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/elf.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/edtest2.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/edtest.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/dwarf.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/configure.ac
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/configure
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config.sub
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config.h.in
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config.guess
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config/lt~obsolete.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config/ltversion.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config/ltsugar.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config/ltoptions.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/config/libtool.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/btest.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/backtrace.h
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/backtrace.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/backtrace-supported.h.in
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/atomic.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/alloc.c
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/aclocal.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/acinclude.m4
[00:03:56] cargo:rerun-if-changed=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/.gitignore
[00:03:56] cargo:rustc-link-lib=static=backtrace
[00:03:56] cargo:rustc-link-search=native=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/libbacktrace/.libs
[00:03:56] running: "sh" "/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/configure" "--with-pic" "--disable-multilib" "--disable-shared" "--disable-host-shared" "--host=x86_64-apple-darwin" "--build=x86_64-apple-darwin"
[00:03:56] checking host system type... x86_64-apple-darwin
[00:03:56] checking target system type... x86_64-apple-darwin
[00:03:56] checking target system type... x86_64-apple-darwin
[00:03:56] checking for x86_64-apple-darwin-gcc... sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang
[00:03:56] checking whether the C compiler works... yes
[00:03:56] checking whether we are cross compiling... no
[00:03:56] checking for suffix of executables... 
[00:03:56] checking for suffix of object files... o
[00:03:56] checking for suffix of object files... o
[00:03:56] checking whether we are using the GNU C compiler... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang accepts -g... yes
[00:03:56] checking for sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang option to accept ISO C89... none needed
[00:03:56] checking how to run the C preprocessor... sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -E
[00:03:56] checking for egrep... /usr/bin/grep -E
[00:03:56] checking for ANSI C header files... yes
[00:03:56] checking for sys/types.h... yes
[00:03:56] checking for sys/stat.h... yes
---
[00:03:56] checking for unistd.h... yes
[00:03:56] checking minix/config.h usability... no
[00:03:56] checking minix/config.h presence... no
[00:03:56] checking for minix/config.h... no
[00:03:56] checking whether it is safe to define __EXTENSIONS__... yes
[00:03:56] checking for a BSD-compatible install... /usr/local/bin/ginstall -c
[00:03:56] checking whether build environment is sane... yes
[00:03:56] checking for a thread-safe mkdir -p... /usr/local/bin/gmkdir -p
[00:03:56] checking for mawk... no
[00:03:56] checking for nawk... no
[00:03:56] checking for awk... awk
[00:03:56] checking whether make sets $(MAKE)... yes
[00:03:56] checking whether make sets $(MAKE)... yes
[00:03:56] checking whether to enable maintainer-specific portions of Makefiles... no
[00:03:56] checking for x86_64-apple-darwin-gcc... (cached) sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang
[00:03:56] checking whether we are using the GNU C compiler... (cached) yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang accepts -g... (cached) yes
[00:03:56] checking for sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang option to accept ISO C89... (cached) none needed
[00:03:56] checking for gawk... (cached) awk
[00:03:56] checking for fgrep... /usr/bin/grep -F
[00:03:56] checking for fgrep... /usr/bin/grep -F
[00:03:56] checking for ld used by sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang... /usr/bin/ld
[00:03:56] checking if the linker (/usr/bin/ld) is GNU ld... no
[00:03:56] checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
[00:03:56] checking the name lister (/usr/bin/nm -B) interface... BSD nm
[00:03:56] checking whether ln -s works... yes
[00:03:56] checking the maximum length of command line arguments... 196608
[00:03:56] checking whether the shell understands some XSI constructs... yes
[00:03:56] checking whether the shell understands "+="... yes
[00:03:56] checking for /usr/bin/ld option to reload object files... -r
[00:03:56] checking how to recognize dependent libraries... pass_all
[00:03:56] checking for x86_64-apple-darwin-ar... ar
[00:03:56] checking for x86_64-apple-darwin-strip... no
[00:03:56] checking for strip... strip
[00:03:56] checking for x86_64-apple-darwin-ranlib... ar s
[00:03:56] checking command to parse /usr/bin/nm -B output from sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang object... ok
[00:03:56] checking for x86_64-apple-darwin-dsymutil... no
[00:03:56] checking for dsymutil... dsymutil
[00:03:56] checking for x86_64-apple-darwin-nmedit... no
[00:03:56] checking for nmedit... nmedit
[00:03:56] checking for x86_64-apple-darwin-lipo... no
[00:03:56] checking for lipo... lipo
[00:03:56] checking for x86_64-apple-darwin-otool... no
[00:03:56] checking for otool... otool
[00:03:56] checking for x86_64-apple-darwin-otool64... no
[00:03:56] checking for otool64... no
[00:03:56] checking for -single_module linker flag... yes
[00:03:56] checking for -exported_symbols_list linker flag... yes
[00:03:56] checking for dlfcn.h... yes
[00:03:56] checking for objdir... .libs
[00:03:56] checking if sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -fno-rtti -fno-exceptions... yes
[00:03:56] checking for sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang option to produce PIC... -fno-common -DPIC
[00:03:56] checking if sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang PIC flag -fno-common -DPIC works... yes
[00:03:56] checking if sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang static flag -static works... no
[00:03:56] checking if sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -c -o file.o... yes
[00:03:56] checking if sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -c -o file.o... (cached) yes
[00:03:56] checking whether the sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang linker (/usr/bin/ld) supports shared libraries... yes
[00:03:56] checking dynamic linker characteristics... darwin dyld
[00:03:56] checking how to hardcode library paths into programs... immediate
[00:03:56] checking whether stripping libraries is possible... yes
[00:03:56] checking if libtool supports shared libraries... yes
[00:03:56] checking whether to build shared libraries... no
[00:03:56] checking whether to build static libraries... yes
[00:03:56] checking for _FILE_OFFSET_BITS value needed for large files... no
[00:03:56] checking unwind.h usability... yes
[00:03:56] checking unwind.h presence... yes
[00:03:56] checking for unwind.h... yes
[00:03:56] checking for unwind.h... yes
[00:03:56] checking for _Unwind_Backtrace... yes
[00:03:56] checking for -funwind-tables option... yes
[00:03:56] checking for -frandom-seed=string option... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -W... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wall... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wwrite-strings... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wstrict-prototypes... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wmissing-prototypes... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wold-style-definition... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wmissing-format-attribute... yes
[00:03:56] checking whether sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang supports -Wcast-qual... yes
[00:03:56] checking for _Unwind_GetIPInfo... yes
[00:03:56] checking __sync extensions... yes
[00:03:56] checking __atomic extensions... yes
[00:03:56] checking output filetype... macho64
[00:03:56] checking sys/mman.h usability... yes
[00:03:56] checking sys/mman.h presence... yes
[00:03:56] checking for sys/mman.h... yes
[00:03:56] checking for mmap... yes
[00:03:56] checking link.h presence... no
[00:03:56] checking for link.h... no
[00:03:56] checking for link.h... no
[00:03:56] checking sys/ldr.h usability... no
[00:03:56] checking sys/ldr.h presence... no
[00:03:56] checking for sys/ldr.h... no
[00:03:56] checking for fcntl... yes
[00:03:56] checking whether strnlen is declared... yes
[00:03:56] checking for readlink... yes
[00:03:56] checking for readlink... yes
[00:03:56] checking for getexecname... no
[00:03:56] checking for clock_gettime... yes
[00:03:56] checking whether -pthread is supported... yes
[00:03:56] checking for compress in -lz... yes
[00:03:56] checking whether --compress-debug-sections is supported... no
[00:03:56] checking for objcopy... no
[00:03:56] checking whether objcopy supports debuglink... no
[00:03:56] checking whether tests can run... yes
[00:03:56] config.status: creating Makefile
[00:03:56] config.status: creating backtrace-supported.h
[00:03:56] config.status: creating config.h
[00:03:56] config.status: executing libtool commands
[00:03:56] config.status: executing libtool commands
[00:03:56] config.status: executing default commands
[00:03:56] running: "make" "INCDIR=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace" "-j" "4"
[00:03:56] /Applications/Xcode.app/Contents/Developer/usr/bin/make  all-am
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=atomic.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o atomic.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/atomic.c
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=dwarf.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o dwarf.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/dwarf.c
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=fileline.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o fileline.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/fileline.c
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=posix.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o posix.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/posix.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=dwarf.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/dwarf.c  -fno-common -DPIC -o dwarf.o
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=atomic.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/atomic.c  -fno-common -DPIC -o atomic.o
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=fileline.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/fileline.c  -fno-common -DPIC -o fileline.o
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=posix.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/posix.c  -fno-common -DPIC -o posix.o
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=print.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o print.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/print.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=print.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/print.c  -fno-common -DPIC -o print.o
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=sort.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o sort.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/sort.c
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=state.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o state.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/state.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=sort.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/sort.c  -fno-common -DPIC -o sort.o
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=state.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/state.c  -fno-common -DPIC -o state.o
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=backtrace.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o backtrace.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/backtrace.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=backtrace.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/backtrace.c  -fno-common -DPIC -o backtrace.o
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=simple.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o simple.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/simple.c
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=macho.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o macho.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=simple.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/simple.c  -fno-common -DPIC -o simple.o
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=macho.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c  -fno-common -DPIC -o macho.o
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=read.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o read.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/read.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=read.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/read.c  -fno-common -DPIC -o read.o
[00:03:56] /bin/sh ./libtool --tag=CC   --mode=compile sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace    -funwind-tables -frandom-seed=alloc.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c -o alloc.lo /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/alloc.c
[00:03:56] libtool: compile:  sccache /Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang -DHAVE_CONFIG_H -I. -I/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace -funwind-tables -frandom-seed=alloc.lo -W -Wall -Wwrite-strings -Wstrict-prototypes -Wmissing-prototypes -Wold-style-definition -Wmissing-format-attribute -Wcast-qual -fPIC -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fvisibility=hidden -O2 -c /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/alloc.c  -fno-common -DPIC -o alloc.o
[00:03:56] 
[00:03:56] 
[00:03:56] command did not execute successfully: "make" "INCDIR=/Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace" "-j" "4"
[00:03:56] 
[00:03:56] 
[00:03:56] 
[00:03:56] --- stderr
[00:03:56] --- stderr
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/configure: line 12006: --add-gnu-debuglink=x: command not found
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:134:1: warning: no previous prototype for function 'macho_file_to_host_u16' [-Wmissing-prototypes]
[00:03:56] macho_file_to_host_u16 (int file_bytes_swapped, uint16_t input)
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:143:1: warning: no previous prototype for function 'macho_file_to_host_u32' [-Wmissing-prototypes]
[00:03:56] macho_file_to_host_u32 (int file_bytes_swapped, uint32_t input)
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:159:1: warning: no previous prototype for function 'macho_file_to_host_u64' [-Wmissing-prototypes]
[00:03:56] macho_file_to_host_u64 (int file_bytes_swapped, uint64_t input)
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:192:1: warning: no previous prototype for function 'macho_get_view' [-Wmissing-prototypes]
[00:03:56] macho_get_view (struct backtrace_state *state, int descriptor,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:225:25: warning: cast from 'const void *' to 'unsigned int *' drops const qualifier [-Wcast-qual]
[00:03:56]   switch (*(uint32_t *) file_header_view.data)
[00:03:56]                         ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:331:29: warning: cast from 'const void *' to 'unsigned int *' drops const qualifier [-Wcast-qual]
[00:03:56]       switch (*(uint32_t *) file_header_view.data)
[00:03:56]                             ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:204:1: warning: no previous prototype for function 'macho_get_commands' [-Wmissing-prototypes]
[00:03:56] macho_get_commands (struct backtrace_state *state, int descriptor,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:436:39: warning: cast from 'const struct load_command *' to 'struct uuid_command *' drops const qualifier [-Wcast-qual]
[00:03:56]               (struct uuid_command *) raw_command;
[00:03:56]                                       ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:392:1: warning: no previous prototype for function 'macho_get_uuid' [-Wmissing-prototypes]
[00:03:56] macho_get_uuid (struct backtrace_state *state ATTRIBUTE_UNUSED,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:502:54: warning: cast from 'const struct load_command *' to 'struct segment_command_64 *' drops const qualifier [-Wcast-qual]
[00:03:56]           raw_segment = (segment_command_native_t *) raw_command;
[00:03:56]                                                      ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:453:1: warning: no previous prototype for function 'macho_get_addr_range' [-Wmissing-prototypes]
[00:03:56] macho_get_addr_range (struct backtrace_state *state ATTRIBUTE_UNUSED,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:551:1: warning: no previous prototype for function 'macho_symbol_type_relevant' [-Wmissing-prototypes]
[00:03:56] macho_symbol_type_relevant (uint8_t type)
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:620:54: warning: cast from 'const struct load_command *' to 'struct symtab_command *' drops const qualifier [-Wcast-qual]
[00:03:56]           symtab_command = (struct symtab_command *) raw_command;
[00:03:56]                                                      ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:560:1: warning: no previous prototype for function 'macho_add_symtab' [-Wmissing-prototypes]
[00:03:56] macho_add_symtab (struct backtrace_state *state,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:813:1: warning: no previous prototype for function 'macho_try_dwarf' [-Wmissing-prototypes]
[00:03:56] macho_try_dwarf (struct backtrace_state *state,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:1045:1: warning: no previous prototype for function 'macho_try_dsym' [-Wmissing-prototypes]
[00:03:56] macho_try_dsym (struct backtrace_state *state,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:1113:1: warning: no previous prototype for function 'macho_add' [-Wmissing-prototypes]
[00:03:56] macho_add (struct backtrace_state *state,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/macho.c:1340:1: error: conflicting types for 'backtrace_initialize'
[00:03:56] backtrace_initialize (struct backtrace_state *state, int descriptor,
[00:03:56] ^
[00:03:56] /Users/travis/build/rust-lang/rust/src/libstd/../libbacktrace/internal.h:270:12: note: previous declaration is here
[00:03:56] extern int backtrace_initialize (struct backtrace_state *state,
[00:03:56]            ^
[00:03:56] 17 warnings and 1 error generated.
[00:03:56] make[2]: *** [macho.lo] Error 1
[00:03:56] make[2]: *** Waiting for unfinished jobs....
[00:03:56] make[1]: *** [all] Error 2
[00:03:56] 
[00:03:56] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:56] expected success, got: exit code: 101
[00:03:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:56] travis_fold:end:stage0-std

[00:03:56] travis_time:end:stage0-std:start=1527219824763113000,finish=1527219894547854000,duration=69784741000


[00:03:56] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:56] Build completed unsuccessfully in 0:01:11
[00:03:56] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3587b159
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:03a05342
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25 19:20 ..
drwx------   2 travis  staff   68 Dec  6 11:00 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:150652b5
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:150652b5:start=1527219896064662000,finish=1527219896085918000,duration=21256000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00403104
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.4

Done. Your build exited with 1.
