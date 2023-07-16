 bash
$ git clean -dfx
...
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --disable-optimize --enable-optimize-cxx --enable-optimize-llvm --disable-debug --disable-debuginfo --disable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
configure: processing ./configure args
configure: 
configure: CFG_DISABLE_VALGRIND_RPASS := 1 
configure: CFG_ENABLE_DEBUGINFO_TESTS := 1 
configure: CFG_ENABLE_LLVM_ASSERTIONS := 1 
configure: CFG_ENABLE_CCACHE    := 1 
configure: CFG_ENABLE_DIST_HOST_ONLY := 1 
configure: CFG_LOCALSTATEDIR    := /var/lib 
configure: CFG_SYSCONFDIR       := /etc 
configure: CFG_DATADIR          := /share 
configure: CFG_INFODIR          := /share/info 
configure: CFG_LLVM_ROOT        :=  
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
configure: CFG_DISABLE_OPTIMIZE := 1 
configure: CFG_ENABLE_LLVM_ASSERTIONS := 1 
configure: CFG_ENABLE_DEBUG_JEMALLOC := 1 
configure: CFG_BOOTSTRAP_KEY    := 20:03:15 
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
configure: CFG_CC               := ccache gcc 
configure: CFG_STDCPP_NAME      := stdc++ 
configure: 
configure: making directories
configure: 
...
configure: configuring LLVM with:
configure: --enable-targets=x86_64 --enable-optimized --enable-assertions --disable-docs --enable-bindings=none --disable-terminfo --disable-zlib --disable-libffi --build=x86_64-unknown-linux-gnu                         --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --with-python=/usr/bin/python2
...
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
configure: CFG_LLVM_BUILD_DIR_x86_64_unknown_linux_gnu := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: CFG_LLVM_INST_DIR_x86_64_unknown_linux_gnu := /home/zazdxscf/build/1nonpkgs/rust/ ...
configure: 
configure: cp -f /home/zazdxscf/build/1nonpkgs/rust/rust/Makefile.in ./Makefile
configure: mv -f config.tmp config.mk
configure: 
configure: configured in release mode. for development consider --enable-debug
configure: 
...
