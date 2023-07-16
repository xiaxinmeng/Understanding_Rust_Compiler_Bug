
   Compiling unwind v0.0.0 (/home/keruspe/Sources/rust/src/libunwind)
     Running `/home/keruspe/Sources/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_std_workspace_core src/tools/rustc-std-workspace-core/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C metadata=72e4d0af026191c2 -C extra-filename=-72e4d0af026191c2 --out-dir /home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps --target i686-unknown-linux-gnu -L dependency=/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps -L dependency=/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern core=/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps/libcore-a33d9cdb501fe3a7.rlib`
     Running `/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/release/build/unwind-a7c5de98df2e44af/build-script-build`
rustc command: "LD_LIBRARY_PATH"="/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/release/deps:/usr/x86_64-pc-linux-gnu/lib" "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--edition=2018" "--crate-name" "rustc_std_workspace_core" "src/tools/rustc-std-workspace-core/lib.rs" "--color" "always" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=2" "-C" "metadata=72e4d0af026191c2-rustc" "-C" "extra-filename=-72e4d0af026191c2" "--out-dir" "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps" "--target" "i686-unknown-linux-gnu" "-L" "dependency=/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps" "-L" "dependency=/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "--extern" "core=/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps/libcore-a33d9cdb501fe3a7.rlib" "-Cdebuginfo=0" "--sysroot" "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2" "-Zexternal-macro-backtrace" "-Cprefer-dynamic" "-Clinker=i686-pc-linux-gnu-cc" "-C" "debug-assertions=n" "-C" "codegen-units=1" "-Zsave-analysis" "-C" "target-feature=-crt-static" "--remap-path-prefix" "/home/keruspe/Sources/rust=/rustc/a73ecb3d9c432f8f53117b1a6b6c209dc802dee7" "-Zunstable-options" "-Z" "force-unstable-if-unmarked"
sysroot: "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2"
libdir: "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2/lib"
error: failed to run custom build command for `unwind v0.0.0 (/home/keruspe/Sources/rust/src/libunwind)`

Caused by:
  process didn't exit successfully: `/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/release/build/unwind-a7c5de98df2e44af/build-script-build` (exit code: 101)
--- stdout
cargo:rerun-if-changed=build.rs
TARGET = Some("i686-unknown-linux-gnu")
OPT_LEVEL = Some("2")
HOST = Some("x86_64-unknown-linux-gnu")
CXX_i686-unknown-linux-gnu = None
CXX_i686_unknown_linux_gnu = None
TARGET_CXX = None
CXX = None
CROSS_COMPILE = None
CXXFLAGS_i686-unknown-linux-gnu = None
CXXFLAGS_i686_unknown_linux_gnu = None
TARGET_CXXFLAGS = None
CXXFLAGS = None
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("false")
CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
running: "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-I" "../llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-o" "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/build/unwind-ea5f1715daebce5e/out/../llvm-project/libunwind/src/Unwind-EHABI.o" "-c" "../llvm-project/libunwind/src/Unwind-EHABI.cpp"
cargo:warning=cc1plus: warning: command line option '-std=c99' is valid for C/ObjC but not for C++
exit code: 0
running: "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-I" "../llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-o" "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/build/unwind-ea5f1715daebce5e/out/../llvm-project/libunwind/src/Unwind-seh.o" "-c" "../llvm-project/libunwind/src/Unwind-seh.cpp"
cargo:warning=cc1plus: warning: command line option '-std=c99' is valid for C/ObjC but not for C++
cargo:warning=In file included from /usr/x86_64-pc-linux-gnu/include/features.h:474,
cargo:warning=                 from /usr/x86_64-pc-linux-gnu/include/assert.h:35,
cargo:warning=                 from ../llvm-project/libunwind/src/config.h:17,
cargo:warning=                 from ../llvm-project/libunwind/src/Unwind-seh.cpp:14:
cargo:warning=/usr/x86_64-pc-linux-gnu/include/gnu/stubs.h:7:11: fatal error: gnu/stubs-32.h: No such file or directory
cargo:warning=    7 | # include <gnu/stubs-32.h>
cargo:warning=      |           ^~~~~~~~~~~~~~~~
cargo:warning=compilation terminated.
exit code: 1

--- stderr
thread 'main' panicked at '

Internal error occurred: Command "c++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-I" "../llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-o" "/home/keruspe/Sources/rust/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/build/unwind-ea5f1715daebce5e/out/../llvm-project/libunwind/src/Unwind-seh.o" "-c" "../llvm-project/libunwind/src/Unwind-seh.cpp" with args "c++" did not execute successfully (status code exit code: 1).

', /home/keruspe/.cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/usr/host/bin/cargo-beta" "build" "--target" "i686-unknown-linux-gnu" "-j" "8" "-v" "--release" "--locked" "--features" "panic-unwind llvm-libunwind backtrace compiler-builtins-c" "--manifest-path" "/home/keruspe/Sources/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/keruspe/Sources/rust/src/bootstrap/bootstrap.py", line 847, in main
    bootstrap(help_triggered)
  File "/home/keruspe/Sources/rust/src/bootstrap/bootstrap.py", line 833, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/keruspe/Sources/rust/src/bootstrap/bootstrap.py", line 141, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/keruspe/Sources/rust/build/bootstrap/debug/bootstrap build libstd
