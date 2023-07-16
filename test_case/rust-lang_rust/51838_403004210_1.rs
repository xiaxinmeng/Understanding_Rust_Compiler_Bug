
--->  Building rust
--->  Building rust for architecture x86_64
Executing:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64" && ulimit -s 32768 && ulimit -a && /usr/bin/make -j2 all RUST_BACKTRACE=1 VERBOSE=1 BOOTSTRAP_ARGS="-vvv -j2"
core file size          (blocks, -c) unlimited
data seg size           (kbytes, -d) unlimited
file size               (blocks, -f) unlimited
max locked memory       (kbytes, -l) unlimited
max memory size         (kbytes, -m) unlimited
open files                      (-n) 10240
pipe size            (512 bytes, -p) 1
stack size              (kbytes, -s) 32768
cpu time               (seconds, -t) unlimited
max user processes              (-u) 709
virtual memory          (kbytes, -v) unlimited
/opt/local/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py build -vvv -j2
running: /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo build --manifest-path /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/Cargo.toml --verbose --verbose --frozen
       Fresh unicode-xid v0.1.0
       Fresh dtoa v0.4.2
       Fresh cc v1.0.10
       Fresh serde v1.0.40
       Fresh num-traits v0.2.2
       Fresh ordermap v0.3.5
       Fresh libc v0.2.40
       Fresh cfg-if v0.1.2
       Fresh fixedbitset v0.1.9
       Fresh itoa v0.4.1
       Fresh getopts v0.2.17
       Fresh lazy_static v0.2.11
       Fresh build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
       Fresh proc-macro2 v0.3.6
       Fresh cmake v0.1.30
       Fresh toml v0.4.6
       Fresh num_cpus v1.8.0
       Fresh time v0.1.39
       Fresh filetime v0.1.15
       Fresh petgraph v0.4.12
       Fresh serde_json v1.0.15
       Fresh quote v0.5.1
       Fresh syn v0.13.1
       Fresh serde_derive_internals v0.23.1
       Fresh serde_derive v1.0.40
       Fresh bootstrap v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.0 secs
running: /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/bootstrap build -vvv -j2
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
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-v" "-v" "--release" "--frozen" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.10
       Fresh core v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libcore)
       Fresh build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
       Fresh cmake v0.1.30
       Fresh compiler_builtins v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/compiler_builtins_shim)
       Fresh libc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/libc_shim)
       Fresh alloc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc)
       Fresh std_unicode v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd_unicode)
       Fresh unwind v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libunwind)
       Fresh alloc_system v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc_system)
       Fresh panic_abort v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libpanic_abort)
       Fresh alloc_jemalloc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/liballoc_jemalloc)
       Fresh panic_unwind v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libpanic_unwind)
       Fresh rustc_asan v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_asan)
       Fresh rustc_tsan v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_tsan)
       Fresh std v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd)
    Finished release [optimized] target(s) in 0.1 secs
not updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1530839264, tv_nsec: 0 } <= SystemTime { tv_sec: 1530839264, tv_nsec: 0 }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
            > Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
            < Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
            > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
            < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libstd }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
        < Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-v" "-v" "--release" "--frozen" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh getopts v0.2.17
       Fresh term v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libterm)
       Fresh test v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libtest)
    Finished release [optimized] target(s) in 0.0 secs
not updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-test/x86_64-apple-darwin/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1530839283, tv_nsec: 0 } <= SystemTime { tv_sec: 1530839284, tv_nsec: 0 }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
          > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
          < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", mode: Libtest }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
      < Test { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Std { target: "x86_64-apple-darwin", compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-v" "-v" "--release" "--frozen" "--features" " jemalloc" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh libc v0.2.40
       Fresh cfg-if v0.1.2
       Fresh stable_deref_trait v1.0.0
       Fresh smallvec v0.6.0
       Fresh bitflags v1.0.1
       Fresh serialize v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libserialize)
       Fresh scoped-tls v0.1.1
       Fresh cc v1.0.10
       Fresh unicode-width v0.1.4
       Fresh termcolor v0.3.6
       Fresh remove_dir_all v0.5.1
       Fresh rustc-demangle v0.1.7
       Fresh graphviz v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libgraphviz)
       Fresh fmt_macros v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libfmt_macros)
       Fresh lazy_static v1.0.0
       Fresh byteorder v1.2.2
       Fresh rustc-serialize v0.3.24
       Fresh lazy_static v0.2.11
       Fresh rustc_platform_intrinsics v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_platform_intrinsics)
       Fresh quick-error v1.2.1
       Fresh ar v0.3.1
       Fresh rand v0.4.2
       Fresh atty v0.2.8
       Fresh log v0.4.1
       Fresh owning_ref v0.3.3
       Fresh backtrace v0.3.6
       Fresh rls-span v0.4.0
       Fresh log_settings v0.1.1
       Fresh humantime v1.1.1
       Fresh parking_lot_core v0.2.14
       Fresh tempdir v0.3.7
       Fresh ena v0.9.2
       Fresh rustc_cratesio_shim v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_cratesio_shim)
       Fresh jobserver v0.1.11
       Fresh rls-data v0.15.0
       Fresh env_logger v0.5.8
       Fresh parking_lot v0.5.5
       Fresh rustc_target v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_target)
       Fresh rustc_apfloat v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_apfloat)
       Fresh miniz-sys v0.1.10
       Fresh rustc_data_structures v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_data_structures)
       Fresh flate2 v1.0.1
       Fresh syntax_pos v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax_pos)
       Fresh arena v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libarena)
       Fresh rustc_errors v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_errors)
       Fresh syntax v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax)
       Fresh proc_macro v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libproc_macro)
       Fresh rustc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc)
       Fresh syntax_ext v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax_ext)
       Fresh rustc_incremental v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_incremental)
       Fresh rustc_mir v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_mir)
       Fresh rustc_typeck v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_typeck)
       Fresh rustc_resolve v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_resolve)
       Fresh rustc_traits v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_traits)
       Fresh rustc_allocator v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_allocator)
       Fresh rustc_metadata v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_metadata)
       Fresh rustc_borrowck v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_borrowck)
       Fresh rustc_lint v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_lint)
       Fresh rustc_trans_utils v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans_utils)
       Fresh rustc_passes v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_passes)
       Fresh rustc_save_analysis v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_save_analysis)
       Fresh rustc_privacy v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_privacy)
       Fresh rustc_plugin v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_plugin)
       Fresh rustc_driver v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_driver)
       Fresh rustc-main v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/rustc)
    Finished release [optimized] target(s) in 0.3 secs
not updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/.librustc.stamp"; contents equal and SystemTime { tv_sec: 1530840700, tv_nsec: 0 } <= SystemTime { tv_sec: 1530840702, tv_nsec: 0 }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target_compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin" }
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
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
Building stage0 codegen artifacts (x86_64-apple-darwin -> x86_64-apple-darwin, llvm)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-v" "-v" "--release" "--frozen" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
       Fresh libc v0.2.40
       Fresh smallvec v0.6.0
       Fresh stable_deref_trait v1.0.0
       Fresh cfg-if v0.1.2
       Fresh bitflags v1.0.1
       Fresh serialize v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libserialize)
       Fresh unicode-width v0.1.4
       Fresh scoped-tls v0.1.1
       Fresh termcolor v0.3.6
       Fresh rustc-demangle v0.1.7
       Fresh remove_dir_all v0.5.1
       Fresh fmt_macros v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libfmt_macros)
       Fresh build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
       Fresh graphviz v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libgraphviz)
       Fresh byteorder v1.2.2
       Fresh lazy_static v0.2.11
       Fresh lazy_static v1.0.0
       Fresh quick-error v1.2.1
       Fresh ar v0.3.1
       Fresh cc v1.0.10
       Fresh rustc_platform_intrinsics v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_platform_intrinsics)
       Fresh rand v0.4.2
       Fresh atty v0.2.8
       Fresh num_cpus v1.8.0
       Fresh owning_ref v0.3.3
       Fresh log v0.4.1
       Fresh backtrace v0.3.6
       Fresh log_settings v0.1.1
       Fresh humantime v1.1.1
       Fresh parking_lot_core v0.2.14
       Fresh tempdir v0.3.7
       Fresh rustc_cratesio_shim v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_cratesio_shim)
       Fresh ena v0.9.2
       Fresh jobserver v0.1.11
       Fresh env_logger v0.5.8
       Fresh parking_lot v0.5.5
       Fresh rustc_target v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_target)
       Fresh rustc_apfloat v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_apfloat)
       Fresh miniz-sys v0.1.10
       Fresh rustc_llvm v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_llvm)
       Fresh rustc_data_structures v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_data_structures)
       Fresh flate2 v1.0.1
       Fresh syntax_pos v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax_pos)
       Fresh arena v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libarena)
       Fresh rustc_errors v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_errors)
       Fresh syntax v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libsyntax)
       Fresh proc_macro v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libproc_macro)
       Fresh rustc v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc)
       Fresh rustc_incremental v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_incremental)
       Fresh rustc_mir v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_mir)
       Fresh rustc_allocator v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_allocator)
       Fresh rustc_trans_utils v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans_utils)
       Fresh rustc_trans v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/librustc_trans)
    Finished release [optimized] target(s) in 0.2 secs
not updating "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/.tmp.stamp"; contents equal and SystemTime { tv_sec: 1530840855, tv_nsec: 0 } <= SystemTime { tv_sec: 1530840859, tv_nsec: 0 }
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
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-apple-darwin" } }
Building stage1 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
running: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-v" "-v" "--release" "--frozen" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.10
   Compiling core v0.0.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libcore)
     Running `/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=d6fce06e1bd3d804 -C extra-filename=-d6fce06e1bd3d804 --out-dir /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -C linker=/opt/local/bin/clang-mp-6.0 -L dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps -L dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/release/deps`
       Fresh build_helper v0.1.0 (file:///opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/build_helper)
       Fresh cmake v0.1.30
rustc command: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1/bin/rustc" "--crate-name" "core" "libcore/lib.rs" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=3" "-C" "metadata=d6fce06e1bd3d804-rustc" "-C" "extra-filename=-d6fce06e1bd3d804" "--out-dir" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps" "--target" "x86_64-apple-darwin" "-C" "linker=/opt/local/bin/clang-mp-6.0" "-L" "dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps" "-L" "dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/release/deps" "--cfg" "stage1" "--sysroot" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1" "-Cprefer-dynamic" "-Clinker=/opt/local/bin/clang-mp-6.0" "-Cdebuginfo=1" "-C" "debug-assertions=n" "-Zsave-analysis" "-Z" "osx-rpath-install-name" "-C" "link-args=-Wl,-rpath,@loader_path/../lib" "-Z" "force-unstable-if-unmarked" "-Dwarnings"
sysroot: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1"
libdir: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1/lib"

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=d6fce06e1bd3d804 -C extra-filename=-d6fce06e1bd3d804 --out-dir /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -C linker=/opt/local/bin/clang-mp-6.0 -L dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps -L dependency=/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/x86_64-apple-darwin/stage1-std/release/deps` (signal: 6, SIGABRT: process abort signal)
command did not execute successfully: "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src/build/stage0-x86_64/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-v" "-v" "--release" "--frozen" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: std::panicking::begin_panic
             at libstd/panicking.rs:402
   5: std::panicking::begin_panic
             at /Users/travis/build/rust-lang/rust/src/libstd/panicking.rs:365
   6: bootstrap::compile::run_cargo
             at bootstrap/compile.rs:1091
   7: bootstrap::sanity::check::{{closure}}
             at bootstrap/compile.rs:109
   8: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:908
   9: bootstrap::compile::copy_apple_sanitizer_dylibs
             at bootstrap/compile.rs:354
  10: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:908
  11: bootstrap::compile::test_cargo
             at bootstrap/compile.rs:470
  12: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:908
  13: bootstrap::compile::compiler_file
             at bootstrap/compile.rs:929
  14: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:908
  15: bootstrap::builder::Builder::run_step_descriptions
             at bootstrap/builder.rs:435
  16: bootstrap::sanity::check::{{closure}}
             at bootstrap/compile.rs:57
  17: bootstrap::builder::StepDescription::maybe_run
             at bootstrap/builder.rs:178
  18: bootstrap::builder::StepDescription::run
             at bootstrap/builder.rs:198
  19: bootstrap::builder::Builder::run_step_descriptions
             at bootstrap/builder.rs:427
  20: bootstrap::builder::Builder::get_step_descriptions
             at bootstrap/builder.rs:417
  21: bootstrap::Crate::local_path
             at bootstrap/lib.rs:435
  22: bootstrap::main
             at bootstrap/bin/main.rs:29
  23: std::rt::lang_start::{{closure}}
             at /Users/travis/build/rust-lang/rust/src/libstd/rt.rs:74
  24: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:306
  25: panic_unwind::dwarf::eh::read_encoded_pointer
             at libpanic_unwind/lib.rs:102
  26: <std::io::Write::write_fmt::Adaptor<'a, T> as core::fmt::Write>::write_str
             at libstd/panicking.rs:285
             at libstd/panic.rs:361
             at libstd/rt.rs:58
  27: std::rt::lang_start
             at /Users/travis/build/rust-lang/rust/src/libstd/rt.rs:74
  28: bootstrap::main
Traceback (most recent call last):
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 817, in <module>
    main()
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 800, in main
    bootstrap(help_triggered)
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 791, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64/build/bootstrap/debug/bootstrap build -vvv -j2
make: *** [all] Error 1
Command failed:  cd "/opt/local/var/macports/build/_opt_macports_lang_rust/rust/work/rustc-1.27.0-src-x86_64" && ulimit -s 32768 && ulimit -a && /usr/bin/make -j2 all RUST_BACKTRACE=1 VERBOSE=1 BOOTSTRAP_ARGS="-vvv -j2"
Exit code: 2
