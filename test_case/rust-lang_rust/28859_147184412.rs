
hsp@mappy $ ./configure                                          git master  ~/co de/rust
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
configure: CFG_LOCALSTATEDIR    := /var/lib
configure: CFG_SYSCONFDIR       := /etc
configure: CFG_DATADIR          := /share
configure: CFG_INFODIR          := /share/info
configure: CFG_LLVM_ROOT        :=
configure: CFG_PYTHON           :=
configure: CFG_JEMALLOC_ROOT    :=
configure: CFG_BUILD            := x86_64-apple-darwin
configure: CFG_ANDROID_CROSS_PATH := /opt/ndk_standalone
configure: CFG_I686_LINUX_ANDROID_NDK :=
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=
configure: CFG_RELEASE_CHANNEL  := dev
configure: CFG_MUSL_ROOT        := /usr/local
configure: CFG_DEFAULT_LINKER   := cc
configure: CFG_DEFAULT_AR       := ar
configure: CFG_BUILD            := x86_64-apple-darwin
configure: CFG_LIBDIR           := /usr/local/lib
configure:
configure: validating ./configure args
configure:
configure: CFG_BOOTSTRAP_KEY    := 04:17:32
configure:
configure: looking for build programs
configure:
configure: CFG_CURLORWGET       := /usr/bin/curl (7.43.0)
configure: CFG_PYTHON           := /usr/local/bin/python2.7
./configure: line 736: [: /Users/hsp/co: binary operator expected
configure: CFG_GIT              := /usr/local/bin/git (2.6.1)
configure: CFG_MD5              := /sbin/md5
configure: CFG_MD5SUM           :=
configure: CFG_HASH_COMMAND     := /sbin/md5 -q | cut -c 1-8
configure: CFG_CLANG            := /usr/bin/clang++ (7.0.0)
configure: CFG_CCACHE           :=
configure: CFG_GCC              := /usr/bin/gcc (7.0.0)
configure: CFG_LD               := /usr/bin/ld
configure: CFG_VALGRIND         :=
configure: CFG_PERF             :=
configure: CFG_ISCC             :=
configure: CFG_ANTLR4           :=
configure: CFG_GRUN             :=
configure: CFG_FLEX             := /usr/bin/flex (2.5.35)
configure: CFG_BISON            := /usr/bin/bison (2.3)
configure: CFG_GDB              :=
configure: CFG_LLDB             := /usr/bin/lldb (340.4.70)
configure: CFG_DISABLE_VALGRIND_RPASS := 1
configure: CFG_LLDB_VERSION     := lldb-340.4.70
configure: CFG_LLDB_PYTHON_DIR  := /Applications/Xcode.app/Contents/Sh ...
configure:
configure: looking for target specific programs
configure:
configure: CFG_ADB              :=
configure:
configure: on OS X >=10.9, forcing use of clang
configure:
configure: CFG_ENABLE_CLANG     := 1
configure: CFG_USING_CLANG      := 1
configure:
configure: found ok version of APPLE CLANG: 7.0.0
configure:
configure: CFG_CC               := clang
configure: CFG_CXX              := clang++
configure: CFG_STDCPP_NAME      := stdc++
configure: error: unsupported target triples "x86_64-apple-darwin" found
