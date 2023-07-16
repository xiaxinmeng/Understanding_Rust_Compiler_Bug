 bash
$ llc --version
LLVM (http://llvm.org/):
  LLVM version 3.8.0svn
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: (unknown)

  Registered Targets:
    amdgcn - AMD GCN GPUs
    cpp    - C++ backend
    r600   - AMD GPUs HD2XXX-HD6XXX
    x86    - 32-bit X86: Pentium-Pro and above
    x86-64 - 64-bit X86: EM64T and AMD64

$ cd /home/zazdxscf/build/1nonpkgs/rust/
//already have: $ wget https://patch-diff.githubusercontent.com/raw/rust-lang/rust/pull/29272.patch
$ cd rust/
$ git clean -dfx
...
$ git reset --hard
$ git pull
$ patch -Np1 -i ../llvm_only_x86_64.patch
$ patch -Np1 -i ../29272.patch
$ time ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --enable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --enable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --llvm-root=/usr --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
configure: looking for configure programs
configure: found program 'cmp'
configure: found program 'mkdir'
configure: found program 'printf'
configure: found program 'cut'
configure: found program 'head'
configure: found program 'grep'
configure: found program 'xargs'
configure: found program 'cp'
configure: found program 'find'
configure: found program 'uname'
configure: found program 'date'
configure: found program 'tr'
configure: found program 'sed'
configure: found program 'file'
configure: found program 'make'
configure: inspecting environment
configure: recreating config.tmp
configure: 
configure: processing ./configure args
configure: 
configure: CFG_ENABLE_DEBUG     := 1 
configure: CFG_DISABLE_VALGRIND_RPASS := 1 
configure: CFG_ENABLE_DEBUGINFO_TESTS := 1 
configure: CFG_ENABLE_LLVM_ASSERTIONS := 1 
configure: CFG_ENABLE_DEBUG_ASSERTIONS := 1 
configure: CFG_ENABLE_CCACHE    := 1 
configure: CFG_ENABLE_RPATH     := 1 
configure: CFG_ENABLE_DIST_HOST_ONLY := 1 
configure: CFG_LOCALSTATEDIR    := /var/lib 
configure: CFG_SYSCONFDIR       := /etc 
configure: CFG_DATADIR          := /share 
configure: CFG_INFODIR          := /share/info 
configure: CFG_LLVM_ROOT        := /usr 
configure: CFG_PYTHON           := /usr/bin/python2 
configure: CFG_JEMALLOC_ROOT    :=  
configure: CFG_BUILD            := x86_64-unknown-linux-gnu 
configure: CFG_ANDROID_CROSS_PATH := /opt/ndk_standalone 
configure: CFG_I686_LINUX_ANDROID_NDK :=  
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=  
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=  
configure: CFG_RELEASE_CHANNEL  := dev 
configure: CFG_MUSL_ROOT        := /usr/local 
configure: CFG_DEFAULT_LINKER   := cc 
configure: CFG_DEFAULT_AR       := ar 
configure: CFG_BUILD            := x86_64-unknown-linux-gnu 
configure: CFG_LIBDIR           := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: 
configure: validating ./configure args
configure: 
configure: debug mode enabled, setting performance options
configure: CFG_ENABLE_LLVM_ASSERTIONS := 1 
configure: CFG_ENABLE_DEBUG_ASSERTIONS := 1 
configure: CFG_ENABLE_DEBUGINFO := 1 
configure: CFG_ENABLE_DEBUG_JEMALLOC := 1 
configure: CFG_BOOTSTRAP_KEY    := 19:36:47 
configure: 
configure: looking for build programs
configure: 
configure: CFG_CURLORWGET       := /usr/bin/curl (7.45.0)
configure: CFG_GIT              := /usr/bin/git (2.6.2)
configure: CFG_MD5              :=  
configure: CFG_MD5SUM           := /usr/bin/md5sum (8.24)
configure: CFG_HASH_COMMAND     := /usr/bin/md5sum | cut -c 1-8 
configure: CFG_CLANG            :=  
configure: CFG_CCACHE           := /usr/bin/ccache (3.2.4)
configure: CFG_GCC              := /usr/bin/gcc (5.2.0)
configure: CFG_LD               := /usr/bin/ld (2.25.1)
configure: CFG_VALGRIND         :=  
configure: CFG_PERF             :=  
configure: CFG_ISCC             :=  
configure: CFG_ANTLR4           :=  
configure: CFG_GRUN             :=  
configure: CFG_FLEX             := /usr/bin/flex (2.5.39)
configure: CFG_BISON            := /usr/bin/bison (3.0.4)
configure: CFG_GDB              := /usr/bin/gdb (7.10)
configure: CFG_LLDB             :=  
configure: CFG_DISABLE_VALGRIND_RPASS := 1 
configure: CFG_GDB_VERSION      := GNU gdb (Gentoo 7.10 vanilla) 7.10 
configure: 
configure: looking for target specific programs
configure: 
configure: CFG_ADB              :=  
configure: 
configure: using custom LLVM at /usr
configure: 
configure: found ok version of LLVM: 3.8.0svn
configure: CFG_CC               := ccache gcc 
configure: CFG_STDCPP_NAME      := stdc++ 
configure: 
configure: making directories
configure: 
configure: mkdir -p doc
configure: mkdir -p doc/std
configure: mkdir -p doc/extra
configure: mkdir -p dl
configure: mkdir -p tmp
configure: mkdir -p dist
configure: mkdir -p x86_64-unknown-linux-gnu/llvm
configure: mkdir -p x86_64-unknown-linux-gnu/rustllvm
configure: mkdir -p x86_64-unknown-linux-gnu/rt
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0
configure: mkdir -p x86_64-unknown-linux-gnu/rt/jemalloc
configure: mkdir -p x86_64-unknown-linux-gnu/rt/compiler-rt
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/isaac
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/sync
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/test
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage0/arch/x86_64
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/isaac
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/sync
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/test
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage1/arch/x86_64
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2/isaac
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2/sync
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2/test
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage2/arch/x86_64
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage3
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage3/isaac
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage3/sync
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage3/test
configure: mkdir -p x86_64-unknown-linux-gnu/rt/stage3/arch/x86_64
configure: mkdir -p x86_64-unknown-linux-gnu/stage0/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage0/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage0/test
configure: mkdir -p x86_64-unknown-linux-gnu/stage1/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage1/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage1/test
configure: mkdir -p x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage2/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage2/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage2/test
configure: mkdir -p x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage3/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage3/lib
configure: mkdir -p x86_64-unknown-linux-gnu/stage3/test
configure: mkdir -p x86_64-unknown-linux-gnu/stage3/lib/rustlib/x86_64-unknown-linux-gnu/bin
configure: mkdir -p x86_64-unknown-linux-gnu/stage3/lib/rustlib/x86_64-unknown-linux-gnu/lib
configure: mkdir -p x86_64-unknown-linux-gnu/test/run-pass
configure: mkdir -p x86_64-unknown-linux-gnu/test/run-pass-valgrind
configure: mkdir -p x86_64-unknown-linux-gnu/test/run-pass-fulldeps
configure: mkdir -p x86_64-unknown-linux-gnu/test/run-fail
configure: mkdir -p x86_64-unknown-linux-gnu/test/run-fail-fulldeps
configure: mkdir -p x86_64-unknown-linux-gnu/test/compile-fail
configure: mkdir -p x86_64-unknown-linux-gnu/test/parse-fail
configure: mkdir -p x86_64-unknown-linux-gnu/test/compile-fail-fulldeps
configure: mkdir -p x86_64-unknown-linux-gnu/test/bench
configure: mkdir -p x86_64-unknown-linux-gnu/test/perf
configure: mkdir -p x86_64-unknown-linux-gnu/test/pretty
configure: mkdir -p x86_64-unknown-linux-gnu/test/debuginfo-gdb
configure: mkdir -p x86_64-unknown-linux-gnu/test/debuginfo-lldb
configure: mkdir -p x86_64-unknown-linux-gnu/test/codegen
configure: mkdir -p x86_64-unknown-linux-gnu/test/rustdoc
configure: 
configure: configuring submodules
configure: 
configure: git: submodule sync
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
configure: git: submodule init
Submodule 'src/llvm' (https://github.com/rust-lang/llvm.git) registered for path 'src/llvm'
configure: git: submodule deinit src/llvm
Cleared directory 'src/llvm'
Submodule 'src/llvm' (https://github.com/rust-lang/llvm.git) unregistered for path 'src/llvm'
configure: git: submodule update
configure: git: submodule foreach sync
Entering 'src/compiler-rt'
Entering 'src/jemalloc'
Entering 'src/rt/hoedown'
Entering 'src/rust-installer'
configure: git: submodule foreach update
configure: git: submodule status
 58ab642c30d9f97735d5745b5d01781ee199c6ae src/compiler-rt (remotes/origin/rust-2015-01-08-do-not-delete)
 e24a1a025a1f214e40eedafe3b9c7b1d69937922 src/jemalloc (3.6.0-158-ge24a1a0)
-cde1ed3196ba9b39bcf028e06e08a8722113a5cb src/llvm
 4638c60dedfa581fd5fa7c6420d8f32274c9ca0b src/rt/hoedown (2.0.0-209-g4638c60)
 c37d3747da75c280237dc2d6b925078e69555499 src/rust-installer (remotes/origin/next-27-gc37d374)
-aed73472416064642911af790b25d57c9390b6c7 src/rust-installer/test/rust-installer-v1
-e577c97b494be2815b215e3042207d6d4b7c5516 src/rust-installer/test/rust-installer-v2
configure: git: submodule clobber
Entering 'src/compiler-rt'
Entering 'src/jemalloc'
Removing VERSION
Entering 'src/rt/hoedown'
Entering 'src/rust-installer'
Entering 'src/compiler-rt'
Entering 'src/jemalloc'
Entering 'src/rt/hoedown'
Entering 'src/rust-installer'
configure: 
configure: looking at LLVM
configure: 
configure: not reconfiguring LLVM, external LLVM root
configure: found program '/usr/bin/FileCheck'
configure: 
configure: writing configuration
configure: 
configure: CFG_SRC_DIR          := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_SRC_DIR_RELATIVE := ./ 
configure: CFG_BUILD_DIR        := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_OSTYPE           := unknown-linux-gnu 
configure: CFG_CPUTYPE          := x86_64 
configure: CFG_CONFIGURE_ARGS   := --prefix=/home/zazdxscf/build/1nonp ...
configure: CFG_PREFIX           := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_HOST             := x86_64-unknown-linux-gnu 
configure: CFG_TARGET           := x86_64-unknown-linux-gnu 
configure: CFG_LIBDIR_RELATIVE  := lib 
configure: CFG_DISABLE_MANAGE_SUBMODULES :=  
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=  
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=  
configure: CFG_I686_LINUX_ANDROID_NDK :=  
configure: CFG_MANDIR           := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_CCACHE_BASEDIR   := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_SRC_DIR     := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_BUILD_DIR_x86_64_unknown_linux_gnu :=  
configure: CFG_LLVM_INST_DIR_x86_64_unknown_linux_gnu := /usr 
configure: 
configure: cp -f /home/zazdxscf/build/1nonpkgs/rust/rust/Makefile.in ./Makefile
configure: mv -f config.tmp config.mk
configure: 
configure: complete
configure: 
configure: run `make help`
configure: 

real    0m14.570s
user    0m2.663s
sys 0m9.207s
