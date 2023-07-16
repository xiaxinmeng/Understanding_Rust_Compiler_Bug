
$ ./configure --enable-cargo-openssl-static
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
configure: recreating config.tmp
configure:
configure: processing ./configure args
configure:
configure: CFG_ENABLE_CARGO_OPENSSL_STATIC := 1
configure: CFG_LOCALSTATEDIR    := /var/lib
configure: CFG_SYSCONFDIR       := /etc
configure: CFG_DATADIR          := /share
configure: CFG_INFODIR          := /share/info
configure: CFG_LLVM_ROOT        :=
configure: CFG_PYTHON           :=
configure: CFG_JEMALLOC_ROOT    :=
configure: CFG_BUILD            :=
configure: CFG_ANDROID_CROSS_PATH :=
configure: CFG_I686_LINUX_ANDROID_NDK :=
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=
configure: CFG_ARMV7_LINUX_ANDROIDEABI_NDK :=
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=
configure: CFG_NACL_CROSS_PATH  :=
configure: CFG_MUSL_ROOT        := /usr/local
configure: CFG_MUSL_ROOT_X86_64 :=
configure: CFG_MUSL_ROOT_I686   :=
configure: CFG_MUSL_ROOT_ARM    :=
configure: CFG_MUSL_ROOT_ARMHF  :=
configure: CFG_MUSL_ROOT_ARMV7  :=
configure: CFG_EXTRA_FILENAME   :=
configure: CFG_QEMU_ARMHF_ROOTFS :=
configure: CFG_RELEASE_CHANNEL  := dev
configure: CFG_DEFAULT_LINKER   := cc
configure: CFG_DEFAULT_AR       := ar
configure: CFG_LIBDIR           := /usr/local/lib
configure:
configure: validating ./configure args
configure:
configure:
configure: looking for build programs
configure:
configure: CFG_CURL             := /usr/bin/curl (7.51.0)
configure: CFG_PYTHON           := /usr/local/bin/python2.7
configure: CFG_DISABLE_VALGRIND_RPASS := 1
configure:
configure: writing configuration
configure:
configure: CFG_SRC_DIR          := /Users/nathan/rust/rust/
configure: CFG_SRC_DIR_RELATIVE := ./
configure: CFG_BUILD_DIR        := /Users/nathan/rust/rust/
configure: CFG_OSTYPE           :=
configure: CFG_CPUTYPE          :=
configure: CFG_CONFIGURE_ARGS   := --enable-cargo-openssl-static
configure: CFG_PREFIX           := /usr/local
configure: CFG_HOST             :=
configure: CFG_TARGET           :=
configure: CFG_LIBDIR_RELATIVE  := lib
configure: CFG_DISABLE_MANAGE_SUBMODULES :=
configure: CFG_AARCH64_LINUX_ANDROID_NDK :=
configure: CFG_ARM_LINUX_ANDROIDEABI_NDK :=
configure: CFG_ARMV7_LINUX_ANDROIDEABI_NDK :=
configure: CFG_I686_LINUX_ANDROID_NDK :=
configure: CFG_NACL_CROSS_PATH  :=
configure: CFG_MANDIR           := /usr/local/share/man
configure: CFG_DOCDIR           := /usr/local/share/doc/rust
configure: CFG_USING_LIBCPP     :=
configure:
configure: cp -f /Users/nathan/rust/rust/src/bootstrap/mk/Makefile.in ./Makefile
configure: mv -f config.tmp config.mk
configure:
configure: configured in release mode. for development consider --enable-debug
configure:
configure: run `python ./x.py --help`
configure:
$ ./x.py build openssl
downloading https://static.rust-lang.org/dist/2017-02-01/rust-std-beta-x86_64-apple-darwin.tar.gz
#################################################################################################################################### 100.0%
extracting /Users/nathan/rust/rust/build/cache/2017-02-01/rust-std-beta-x86_64-apple-darwin.tar.gz
downloading https://static.rust-lang.org/dist/2017-02-01/rustc-beta-x86_64-apple-darwin.tar.gz
#################################################################################################################################### 100.0%
extracting /Users/nathan/rust/rust/build/cache/2017-02-01/rustc-beta-x86_64-apple-darwin.tar.gz
downloading https://s3.amazonaws.com/rust-lang-ci/cargo-builds/407edef22e894266eb562618cba5ca9757051946/cargo-nightly-x86_64-apple-darwin.tar.gz
#################################################################################################################################### 100.0%
extracting /Users/nathan/rust/rust/build/cache/407edef22e894266eb562618cba5ca9757051946/cargo-nightly-x86_64-apple-darwin.tar.gz
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading toml v0.1.30
 Downloading rustc-serialize v0.3.23
 Downloading num_cpus v0.2.13
 Downloading libc v0.2.21
 Downloading gcc v0.3.44
 Downloading cmake v0.1.22
 Downloading filetime v0.1.10
 Downloading getopts v0.2.14
   Compiling getopts v0.2.14
   Compiling gcc v0.3.44
   Compiling libc v0.2.21
   Compiling rustc-serialize v0.3.23
   Compiling num_cpus v0.2.13
   Compiling filetime v0.1.10
   Compiling build_helper v0.1.0 (file:///Users/nathan/rust/rust/src/build_helper)
   Compiling cmake v0.1.22
   Compiling toml v0.1.30
   Compiling bootstrap v0.0.0 (file:///Users/nathan/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 11.63 secs
Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'cargo'
Cloning into '/Users/nathan/rust/rust/cargo'...
Submodule path 'cargo': checked out 'c995e9eb5acf3976ae8674a0dc6d9e958053d9fd'
Submodule 'src/compiler-rt' (https://github.com/rust-lang/compiler-rt.git) registered for path 'src/compiler-rt'
Cloning into '/Users/nathan/rust/rust/src/compiler-rt'...
Submodule path 'src/compiler-rt': checked out 'd30da544a8afc5d78391dee270bdf40e74a215d3'
Submodule 'book' (https://github.com/rust-lang/book) registered for path 'src/doc/book'
Cloning into '/Users/nathan/rust/rust/src/doc/book'...
Submodule path 'src/doc/book': checked out 'e6d6caab41471f7115a621029bd428a812c5260e'
Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
Cloning into '/Users/nathan/rust/rust/src/doc/nomicon'...
Submodule path 'src/doc/nomicon': checked out 'd08fe97d12b41c1ed8cc7701e545864132783941'
Submodule 'reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
Cloning into '/Users/nathan/rust/rust/src/doc/reference'...
Submodule path 'src/doc/reference': checked out '516549972d61c8946542d1a34afeae97167ff77b'
Submodule 'src/jemalloc' (https://github.com/rust-lang/jemalloc.git) registered for path 'src/jemalloc'
Cloning into '/Users/nathan/rust/rust/src/jemalloc'...
Submodule path 'src/jemalloc': checked out '11bfb0dcf85f7aa92abd30524bb1e42e18d108c6'
Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
Cloning into '/Users/nathan/rust/rust/src/liblibc'...
Submodule path 'src/liblibc': checked out '64d954c6a76e896fbf7ed5c17e77c40e388abe84'
Submodule 'src/llvm' (https://github.com/rust-lang/llvm.git) registered for path 'src/llvm'
Cloning into '/Users/nathan/rust/rust/src/llvm'...
Submodule path 'src/llvm': checked out 'd5ef27a79661d4f0d57d7b7d2cdbe9204f790a4a'
Submodule 'src/rt/hoedown' (https://github.com/rust-lang/hoedown.git) registered for path 'src/rt/hoedown'
Cloning into '/Users/nathan/rust/rust/src/rt/hoedown'...
Submodule path 'src/rt/hoedown': checked out 'da282f1bb7277b4d30fa1599ee29ad8eb4dd2a92'
Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/rust-installer'
Cloning into '/Users/nathan/rust/rust/src/rust-installer'...
Submodule path 'src/rust-installer': checked out '4f994850808a572e2cc8d43f968893c8e942e9bf'
 Downloading atty v0.2.2
 Downloading rls-data v0.1.0
 Downloading toml v0.3.1
 Downloading utf8-ranges v1.0.0
 Downloading lazy_static v0.2.4
 Downloading winapi v0.2.8
 Downloading mdbook v0.0.18
 Downloading void v1.0.2
 Downloading serde_json v0.9.9
 Downloading winapi-build v0.1.1
 Downloading thread_local v0.3.3
 Downloading unreachable v0.1.1
 Downloading serde v0.9.11
 Downloading term_size v0.2.3
 Downloading handlebars v0.25.1
 Downloading rls-span v0.1.0
 Downloading quick-error v1.1.0
 Downloading aho-corasick v0.6.2
 Downloading thread-id v3.0.0
 Downloading clap v2.21.1
 Downloading bitflags v0.8.0
 Downloading memchr v1.0.1
 Downloading open v1.2.0
 Downloading pulldown-cmark v0.0.8
 Downloading env_logger v0.4.2
 Downloading regex v0.2.1
 Downloading env_logger v0.3.5
 Downloading bitflags v0.5.0
 Downloading vec_map v0.7.0
 Downloading pest v0.3.3
 Downloading kernel32-sys v0.2.2
 Downloading log v0.3.7
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 5184k  100 5184k    0     0  1596k      0  0:00:03  0:00:03 --:--:-- 1596k
Configuring openssl for x86_64-apple-darwin
Building openssl for x86_64-apple-darwin
Installing openssl for x86_64-apple-darwin
Build completed successfully in 0:08:54
