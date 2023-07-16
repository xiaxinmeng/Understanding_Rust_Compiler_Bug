
--->  Extracting rustc-1.27.0-src.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/rustc-1.27.0-src.tar.gz' | /usr/bin/gnutar -xf -
--->  Extracting rust-std-1.26.0-x86_64-apple-darwin.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/rust-std-1.26.0-x86_64-apple-darwin.tar.gz' | /usr/bin/gnutar -xf -
--->  Extracting rustc-1.26.0-x86_64-apple-darwin.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/rustc-1.26.0-x86_64-apple-darwin.tar.gz' | /usr/bin/gnutar -xf -
--->  Extracting cargo-0.27.0-x86_64-apple-darwin.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/cargo-0.27.0-x86_64-apple-darwin.tar.gz' | /usr/bin/gnutar -xf -
--->  Extracting rust-std-1.26.0-i686-apple-darwin.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/rust-std-1.26.0-i686-apple-darwin.tar.gz' | /usr/bin/gnutar -xf -
--->  Extracting rustc-1.26.0-i686-apple-darwin.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/rustc-1.26.0-i686-apple-darwin.tar.gz' | /usr/bin/gnutar -xf -
--->  Extracting cargo-0.27.0-i686-apple-darwin.tar.gz
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work" && /usr/bin/gzip -dc '/opt/local/var/macports/distfiles/rust/cargo-0.27.0-i686-apple-darwin.tar.gz' | /usr/bin/gnutar -xf -
Warning: The following existing file was hidden from the build system by trace mode:
  /opt/local/bin/lipo
--->  Applying patches to rust
--->  Applying patch-src-librustc-llvm-lib.diff
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src" && /usr/bin/patch -p0 < '/opt/macports/lang/rust/files/patch-src-librustc-llvm-lib.diff'
patching file src/librustc_llvm/lib.rs
Hunk #1 succeeded at 421 (offset 1 line).
--->  Configuring rust
--->  Configuring rust for architecture x86_64
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64" && ./configure --prefix=/opt/local --enable-vendor --default-linker=/opt/local/bin/clang-mp-6.0 --disable-codegen-tests --disable-docs --release-channel=stable --set=target.x86_64-apple-darwin.cc=/opt/local/bin/clang-mp-6.0 --set=target.x86_64-apple-darwin.cxx=/opt/local/bin/clang++-mp-6.0 --set=target.x86_64-apple-darwin.linker=/opt/local/bin/clang-mp-6.0 --set=target.i686-apple-darwin.cc=/opt/local/bin/clang-mp-6.0 --set=target.i686-apple-darwin.cxx=/opt/local/bin/clang++-mp-6.0 --set=target.i686-apple-darwin.linker=/opt/local/bin/clang-mp-6.0 --llvm-root=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/llvm-6.0-x86_64 --build=x86_64-apple-darwin --local-rust-root=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64
configure: processing command line
configure:
configure: rust.default-linker  := /opt/local/bin/clang-mp-6.0
configure: build.vendor         := True
configure: rust.channel         := stable
configure: build.docs           := False
configure: rust.codegen-tests   := False
configure: install.prefix       := /opt/local
configure: target.x86_64-apple-darwin.cc := /opt/local/bin/clang-mp-6.0
configure: target.x86_64-apple-darwin.cxx := /opt/local/bin/clang++-mp-6.0
configure: target.x86_64-apple-darwin.linker := /opt/local/bin/clang-mp-6.0
configure: target.i686-apple-darwin.cc := /opt/local/bin/clang-mp-6.0
configure: target.i686-apple-darwin.cxx := /opt/local/bin/clang++-mp-6.0
configure: target.i686-apple-darwin.linker := /opt/local/bin/clang-mp-6.0
configure: build.build          := x86_64-apple-darwin
configure: build.rustc          := /opt/local/var/macports/build/_opt_macports_la ...
configure: build.cargo          := /opt/local/var/macports/build/_opt_macports_la ...
configure: target.x86_64-apple-darwin.llvm-config := /opt/local/var/macports/buil ...
configure: build.configure-args := ['--prefix=/opt/local', '--enable-vendor', '-- ...
configure:
configure: writing `config.toml` in current directory
configure:
configure: run `python /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/x.py --help`
configure:
--->  Configuring rust for architecture i386
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-i386" && ./configure --prefix=/opt/local --enable-vendor --default-linker=/opt/local/bin/clang-mp-6.0 --disable-codegen-tests --disable-docs --release-channel=stable --set=target.x86_64-apple-darwin.cc=/opt/local/bin/clang-mp-6.0 --set=target.x86_64-apple-darwin.cxx=/opt/local/bin/clang++-mp-6.0 --set=target.x86_64-apple-darwin.linker=/opt/local/bin/clang-mp-6.0 --set=target.i686-apple-darwin.cc=/opt/local/bin/clang-mp-6.0 --set=target.i686-apple-darwin.cxx=/opt/local/bin/clang++-mp-6.0 --set=target.i686-apple-darwin.linker=/opt/local/bin/clang-mp-6.0 --llvm-root=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/llvm-6.0-i386 --build=i686-apple-darwin --local-rust-root=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-i686
configure: processing command line
configure:
configure: rust.default-linker  := /opt/local/bin/clang-mp-6.0
configure: build.vendor         := True
configure: rust.channel         := stable
configure: build.docs           := False
configure: rust.codegen-tests   := False
configure: install.prefix       := /opt/local
configure: target.x86_64-apple-darwin.cc := /opt/local/bin/clang-mp-6.0
configure: target.x86_64-apple-darwin.cxx := /opt/local/bin/clang++-mp-6.0
configure: target.x86_64-apple-darwin.linker := /opt/local/bin/clang-mp-6.0
configure: target.i686-apple-darwin.cc := /opt/local/bin/clang-mp-6.0
configure: target.i686-apple-darwin.cxx := /opt/local/bin/clang++-mp-6.0
configure: target.i686-apple-darwin.linker := /opt/local/bin/clang-mp-6.0
configure: build.build          := i686-apple-darwin
configure: build.rustc          := /opt/local/var/macports/build/_opt_macports_la ...
configure: build.cargo          := /opt/local/var/macports/build/_opt_macports_la ...
configure: target.i686-apple-darwin.llvm-config := /opt/local/var/macports/build/ ...
configure: build.configure-args := ['--prefix=/opt/local', '--enable-vendor', '-- ...
configure:
configure: writing `config.toml` in current directory
configure:
configure: run `python /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-i386/x.py --help`
configure:
xinstall: mkdir /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/.home/.cargo
Warning: The following file inside the MacPorts prefix not installed by a port was accessed:
  /opt/local/Library/Frameworks/Python.framework/Versions/2.7/lib/python2.7/site-packages/singledispatch-3.4.0.3-py2.7.egg
--->  Building rust
--->  Building rust for architecture x86_64
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64" && /usr/bin/make -j2 -w all VERBOSE=1 BOOTSTRAP_ARGS="-v -j2"
make: Entering directory `/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64'
/opt/local/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py build -v -j2
running: /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo build --manifest-path /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/Cargo.toml --frozen
   Compiling unicode-xid v0.1.0
   Compiling serde v1.0.40
   Compiling num-traits v0.2.2
   Compiling libc v0.2.40
   Compiling dtoa v0.4.2
   Compiling cfg-if v0.1.2
   Compiling fixedbitset v0.1.9
   Compiling cc v1.0.10
   Compiling ordermap v0.3.5
   Compiling itoa v0.4.1
   Compiling build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
   Compiling getopts v0.2.17
   Compiling lazy_static v0.2.11
   Compiling proc-macro2 v0.3.6
   Compiling time v0.1.39
   Compiling num_cpus v1.8.0
   Compiling filetime v0.1.15
   Compiling cmake v0.1.30
   Compiling petgraph v0.4.12
   Compiling toml v0.4.6
   Compiling serde_json v1.0.15
   Compiling quote v0.5.1
   Compiling syn v0.13.1
   Compiling serde_derive_internals v0.23.1
   Compiling serde_derive v1.0.40
   Compiling bootstrap v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap)
    Finished dev [unoptimized] target(s) in 143.9 secs
running: /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/bootstrap build -v -j2
finding compilers
CC_x86_64-apple-darwin = "/opt/local/bin/clang-mp-6.0"
CFLAGS_x86_64-apple-darwin = ["-ffunction-sections", "-fdata-sections", "-fPIC", "--target=x86_64-apple-darwin", "-stdlib=libc++"]
AR_x86_64-apple-darwin = "/opt/local/bin/ar"
CXX_x86_64-apple-darwin = "/opt/local/bin/clang++-mp-6.0"
running sanity check
learning about cargo
> Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
    < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
    > Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        > Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release
          > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
            > Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
            < Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
            > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
            < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        < Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
          < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      < Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
        < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    < Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", backend: "llvm" }
      c Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > Llvm { target: "x86_64-apple-darwin", emscripten: false }
      < Llvm { target: "x86_64-apple-darwin", emscripten: false }
    < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", backend: "llvm" }
    > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    > Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  < Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  > Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    > Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      > Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        > StdLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          > CleanTools { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
          < CleanTools { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
        < StdLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      < Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      > TestLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        > CleanTools { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
        < CleanTools { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
      < TestLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    < Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      > CleanTools { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
      < CleanTools { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
    < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
  < Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  > CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", backend: "llvm" }
    c Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Llvm { target: "x86_64-apple-darwin", emscripten: false }
  < CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", backend: "llvm" }
  > Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  < Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
  > Libdir { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  < Libdir { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
< Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
> Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  > StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
  < StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  c Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  > StdLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    > CleanTools { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < CleanTools { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
  < StdLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
< Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
> Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  c Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  > TestLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    > CleanTools { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < CleanTools { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
  < TestLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
< Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
> Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  c Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    > CleanTools { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < CleanTools { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
  < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
< Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
> Rustdoc { host: "x86_64-apple-darwin" }
  c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
< Rustdoc { host: "x86_64-apple-darwin" }
> Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" } }
  > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
    < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
    > Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        > Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--frozen" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd/Cargo.toml" "--message-format" "json"
   Compiling cc v1.0.10
   Compiling core v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libcore)
   Compiling build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
   Compiling unwind v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libunwind)
   Compiling compiler_builtins v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.30
   Compiling alloc_jemalloc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd)
   Compiling rustc_tsan v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_tsan)
   Compiling rustc_asan v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_asan)
   Compiling libc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc)
   Compiling std_unicode v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd_unicode)
   Compiling alloc_system v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc_system)
   Compiling panic_abort v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libpanic_abort)
   Compiling panic_unwind v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libpanic_unwind)
    Finished release [optimized] target(s) in 174.6 secs
updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/.libstd.stamp" as "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/libstd-4cea9531370d7b69.rlib" changed
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
            > Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
            < Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
            > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-tools
            < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        < Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-test/x86_64-apple-darwin/release
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--frozen" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libtest/Cargo.toml" "--message-format" "json"
   Compiling term v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libterm)
   Compiling getopts v0.2.17
   Compiling test v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libtest)
    Finished release [optimized] target(s) in 19.92 secs
updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-test/x86_64-apple-darwin/release/.libtest.stamp" as "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-test/x86_64-apple-darwin/release/deps/libtest-e021906026881646.rlib" changed
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-tools
          < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      < Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--frozen" "--features" " jemalloc" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/Cargo.toml" "--message-format" "json"
   Compiling libc v0.2.40
   Compiling smallvec v0.6.0
   Compiling cfg-if v0.1.2
   Compiling stable_deref_trait v1.0.0
   Compiling bitflags v1.0.1
   Compiling serialize v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libserialize)
   Compiling unicode-width v0.1.4
   Compiling cc v1.0.10
   Compiling rustc_target v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_target)
   Compiling scoped-tls v0.1.1
   Compiling syntax v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax)
   Compiling termcolor v0.3.6
   Compiling remove_dir_all v0.5.1
   Compiling rustc-demangle v0.1.7
   Compiling rustc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc)
   Compiling byteorder v1.2.2
   Compiling graphviz v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libgraphviz)
   Compiling fmt_macros v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libfmt_macros)
   Compiling lazy_static v1.0.0
   Compiling rustc-serialize v0.3.24
   Compiling lazy_static v0.2.11
   Compiling rustc_metadata v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_metadata)
   Compiling rustc_incremental v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_incremental)
   Compiling rustc_platform_intrinsics v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_platform_intrinsics)
   Compiling quick-error v1.2.1
   Compiling ar v0.3.1
   Compiling rustc_driver v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_driver)
   Compiling log v0.4.1
   Compiling owning_ref v0.3.3
   Compiling rand v0.4.2
   Compiling atty v0.2.8
   Compiling miniz-sys v0.1.10
   Compiling backtrace v0.3.6
   Compiling log_settings v0.1.1
   Compiling rls-span v0.4.0
   Compiling humantime v1.1.1
   Compiling rustc_cratesio_shim v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_cratesio_shim)
   Compiling ena v0.9.2
   Compiling jobserver v0.1.11
   Compiling parking_lot_core v0.2.14
   Compiling tempdir v0.3.7
   Compiling rls-data v0.15.0
   Compiling env_logger v0.5.8
   Compiling rustc_apfloat v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_apfloat)
   Compiling parking_lot v0.5.5
   Compiling flate2 v1.0.1
   Compiling rustc_data_structures v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_data_structures)
   Compiling syntax_pos v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax_pos)
   Compiling arena v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libarena)
   Compiling rustc_errors v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_errors)
   Compiling proc_macro v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libproc_macro)
   Compiling syntax_ext v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax_ext)
   Compiling rustc_mir v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_mir)
   Compiling rustc_typeck v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_typeck)
   Compiling rustc_traits v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_traits)
   Compiling rustc_resolve v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_resolve)
   Compiling rustc_allocator v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_allocator)
   Compiling rustc_plugin v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_plugin)
   Compiling rustc_trans_utils v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans_utils)
   Compiling rustc_borrowck v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_borrowck)
   Compiling rustc_lint v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_lint)
   Compiling rustc_passes v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_passes)
   Compiling rustc_privacy v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_privacy)
   Compiling rustc_save_analysis v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_save_analysis)
   Compiling rustc-main v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc)
    Finished release [optimized] target(s) in 1395.61 secs
updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/.librustc.stamp" as "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_driver-5f76c8601eba04f6.dylib" changed
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-tools
        < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Librustc }
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    < Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", backend: "llvm" }
      c Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > Llvm { target: "x86_64-apple-darwin", emscripten: false }
      < Llvm { target: "x86_64-apple-darwin", emscripten: false }
Building stage0 codegen artifacts (x86_64-apple-darwin -> x86_64-apple-darwin, llvm)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--frozen" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
   Compiling build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
   Compiling rustc_trans v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans)
   Compiling cc v1.0.10
   Compiling num_cpus v1.8.0
   Compiling rustc_llvm v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_llvm)
    Finished release [optimized] target(s) in 147.62 secs
updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/.tmp.stamp" as "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_trans-960700e10aaeb56b.dylib" changed
    < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", backend: "llvm" }
Assembling stage1 compiler (x86_64-apple-darwin)
    > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    > Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    < Libdir { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  < Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
  > Rustc { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
    > Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
      > Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
Dirty - /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
Building stage1 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--frozen" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd/Cargo.toml" "--message-format" "json"
   Compiling cc v1.0.10
   Compiling core v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libcore)
   Compiling build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
   Compiling unwind v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libunwind)
   Compiling compiler_builtins v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/compiler_builtins_shim)
   Compiling cmake v0.1.30
   Compiling alloc_jemalloc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd)
   Compiling rustc_tsan v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_tsan)
   Compiling rustc_asan v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_asan)

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=d6fce06e1bd3d804 -C extra-filename=-d6fce06e1bd3d804 --out-dir /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -C linker=/opt/local/bin/clang-mp-6.0 -L dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps -L dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/release/deps` (signal: 6, SIGABRT: process abort signal)
command did not execute successfully: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "--release" "--frozen" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Traceback (most recent call last):
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 817, in <module>
    main()
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 800, in main
    bootstrap(help_triggered)
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 791, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/bootstrap build -v -j2
make: *** [all] Error 1
make: Leaving directory `/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64'
Command failed:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64" && /usr/bin/make -j2 -w all VERBOSE=1 BOOTSTRAP_ARGS="-v -j2"
Exit code: 2
