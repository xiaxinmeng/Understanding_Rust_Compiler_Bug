
Updating submodules
running: git submodule -q sync
running: git submodule update --init --recursive src/llvm src/rt/hoedown src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/doc/book src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/binaryen
running: git submodule -q foreach git reset -q --hard
running: git submodule -q foreach git clean -qdfx
running: C:\Users\zilbuz\projets\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path C:\Users\zilbuz\projets\rust\src/bootstrap/Cargo.toml --verbose
       Fresh serde v1.0.19
       Fresh cfg-if v0.1.2
       Fresh libc v0.2.33
       Fresh quote v0.3.15
       Fresh num-traits v0.1.40
       Fresh lazy_static v0.2.9
       Fresh cc v1.0.3
       Fresh dtoa v0.4.2
       Fresh itoa v0.3.4
       Fresh getopts v0.2.15
       Fresh unicode-xid v0.0.4
       Fresh toml v0.4.5
       Fresh filetime v0.1.14
       Fresh num_cpus v1.7.0
       Fresh cmake v0.1.26
       Fresh serde_json v1.0.6
       Fresh synom v0.11.3
       Fresh build_helper v0.1.0 (file:///C:/Users/zilbuz/projets/rust/src/build_helper)
       Fresh syn v0.11.11
       Fresh serde_derive_internals v0.17.0
       Fresh serde_derive v1.0.19
       Fresh bootstrap v0.0.0 (file:///C:/Users/zilbuz/projets/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.0 secs
running: C:\Users\zilbuz\projets\rust\build\bootstrap\debug\bootstrap build -v --stage 0 src\librustc
finding compilers
CC_x86_64-pc-windows-msvc = "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\VC\\Tools\\MSVC\\14.11.25503\\bin\\HostX64\\x64\\cl.exe"
CXX_x86_64-pc-windows-msvc = "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\VC\\Tools\\MSVC\\14.11.25503\\bin\\HostX64\\x64\\cl.exe"
running sanity check
auto-detected local-rebuild 1.23.0-nightly
learning about cargo
> Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
< Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
> Rustc { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
  > Test { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
    > Std { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
      > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
      < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
Building stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
      > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
      < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
running: "C:\\Users\\zilbuz\\projets\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--features" "panic-unwind backtrace force_alloc_system" "--manifest-path" "C:\\Users\\zilbuz\\projets\\rust\\src/libstd/Cargo.toml" "--message-format" "json"
    Finished release [optimized] target(s) in 0.0 secs
not updating "C:\\Users\\zilbuz\\projets\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\.libstd.stamp"; contents equal and 13155929110.614656800s <= 13155929110.697176400s
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
      > StdLink { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
Copying stage0 std from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
        > Libdir { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
        < Libdir { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
        > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd }
        < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libstd }
      < StdLink { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
    < Std { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
Building stage0 test artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
running: "C:\\Users\\zilbuz\\projets\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--manifest-path" "C:\\Users\\zilbuz\\projets\\rust\\src/libtest/Cargo.toml" "--message-format" "json"
    Finished release [optimized] target(s) in 0.0 secs
not updating "C:\\Users\\zilbuz\\projets\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\.libtest.stamp"; contents equal and 13155929121.138725900s <= 13155929121.210243200s
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
    > TestLink { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
Copying stage0 test from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
      c Libdir { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
      > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libtest }
      < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc", mode: Libtest }
    < TestLink { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
  < Test { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" }, target: "x86_64-pc-windows-msvc" }
  > Llvm { target: "x86_64-pc-windows-msvc" }
  < Llvm { target: "x86_64-pc-windows-msvc" }
  c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
  c Std { target: "x86_64-pc-windows-msvc", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
Building stage0 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
  c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-msvc" } }
running: "C:\\Users\\zilbuz\\projets\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--features" " llvm" "--manifest-path" "C:\\Users\\zilbuz\\projets\\rust\\src/rustc/Cargo.toml" "--message-format" "json"
   Compiling rustc_binaryen v0.0.0 (file:///C:/Users/zilbuz/projets/rust/src/librustc_binaryen)
   Compiling rustc_resolve v0.0.0 (file:///C:/Users/zilbuz/projets/rust/src/librustc_resolve)
   Compiling rustc_allocator v0.0.0 (file:///C:/Users/zilbuz/projets/rust/src/librustc_allocator)
   Compiling rustc_metadata v0.0.0 (file:///C:/Users/zilbuz/projets/rust/src/librustc_metadata)
error: failed to run custom build command for `rustc_binaryen v0.0.0 (file:///C:/Users/zilbuz/projets/rust/src/librustc_binaryen)`
process didn't exit successfully: `C:\Users\zilbuz\projets\rust\build\x86_64-pc-windows-msvc\stage0-rustc\release\build\rustc_binaryen-5b38d8ef9a1be3fe\build-script-build` (exit code: 101)
--- stderr
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1

build script failed, must exit now', C:\Users\zilbuz\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.26\src\lib.rs:599:4
stack backtrace:
   0: std::sys_common::backtrace::_print
             at C:\projects\rust\src\libstd\sys_common\backtrace.rs:91
   1: std::panicking::default_hook::{{closure}}
             at C:\projects\rust\src\libstd\panicking.rs:383
   2: std::panicking::default_hook
             at C:\projects\rust\src\libstd\panicking.rs:397
   3: std::panicking::rust_panic_with_hook
             at C:\projects\rust\src\libstd\panicking.rs:577
   4: std::panicking::begin_panic<alloc::string::String>
             at C:\projects\rust\src\libstd\panicking.rs:538
   5: std::panicking::begin_panic_fmt
             at C:\projects\rust\src\libstd\panicking.rs:522
   6: cmake::Config::build
   7: cmake::Config::build
   8: cmake::Config::build
   9: POW10
  10: panic_unwind::__rust_maybe_catch_panic
             at C:\projects\rust\src\libpanic_unwind\lib.rs:101
  11: std::rt::lang_start
             at C:\projects\rust\src\libstd\rt.rs:51
  12: __scrt_common_main_seh
             at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:283
  13: BaseThreadInitThunk

warning: build failed, waiting for other jobs to finish...
error: build failed
thread 'main' panicked at 'command did not execute successfully: "C:\\Users\\zilbuz\\projets\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-j" "4" "--release" "--features" " llvm" "--manifest-path" "C:\\Users\\zilbuz\\projets\\rust\\src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src\bootstrap\compile.rs:882:8
stack backtrace:
   0: std::sys_common::backtrace::_print
             at C:\projects\rust\src\libstd\sys_common\backtrace.rs:91
   1: std::panicking::default_hook::{{closure}}
             at C:\projects\rust\src\libstd\panicking.rs:383
   2: std::panicking::default_hook
             at C:\projects\rust\src\libstd\panicking.rs:397
   3: std::panicking::rust_panic_with_hook
             at C:\projects\rust\src\libstd\panicking.rs:577
   4: std::panicking::begin_panic<alloc::string::String>
             at C:\projects\rust\src\libstd\panicking.rs:538
   5: std::panicking::begin_panic_fmt
             at C:\projects\rust\src\libstd\panicking.rs:522
   6: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run::{{closure}}
   7: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
   8: bootstrap::builder::Builder::ensure
   9: <bootstrap::compile::Rustc as bootstrap::builder::Step>::make_run
  10: <bootstrap::builder::Builder<'a> as core::ops::deref::Deref>::deref
  11: <bootstrap::builder::Builder<'a> as core::ops::deref::Deref>::deref
  12: bootstrap::builder::Builder::run
  13: bootstrap::Build::build
  14: core::ptr::<impl *mut T>::is_null
  15: panic_unwind::__rust_maybe_catch_panic
             at C:\projects\rust\src\libpanic_unwind\lib.rs:101
  16: std::rt::lang_start
             at C:\projects\rust\src\libstd\rt.rs:51
  17: main
  18: __scrt_common_main_seh
             at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:283
  19: BaseThreadInitThunk
Traceback (most recent call last):
  File "x.py", line 20, in <module>
    bootstrap.main()
  File "C:\Users\zilbuz\projets\rust\src\bootstrap\bootstrap.py", line 758, in main
    bootstrap()
  File "C:\Users\zilbuz\projets\rust\src\bootstrap\bootstrap.py", line 749, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "C:\Users\zilbuz\projets\rust\src\bootstrap\bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: C:\Users\zilbuz\projets\rust\build\bootstrap\debug\bootstrap build -v --stage 0 src\librustc
