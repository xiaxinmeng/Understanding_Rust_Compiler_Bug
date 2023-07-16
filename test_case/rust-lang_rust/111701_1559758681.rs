plain
Updating files:  98% (40557/41384)
Updating files:  99% (40971/41384)
Updating files: 100% (41384/41384)
Updating files: 100% (41384/41384), done.
HEAD is now at a1bebd5c Merge fd1c67203d92019fc1bbd77ce4a1df95974fe459 into b08148f6a76010ea3d4e91d61245aa7aac59e4b4
[command]/usr/local/bin/git log -1 --format='%H'
'a1bebd5c848bfc2516cda59be6bb55b320c310d9'
'a1bebd5c848bfc2516cda59be6bb55b320c310d9'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-apple-various
  CARGO_REGISTRIES_CRATES_IO_PROTOCOL: sparse
  SCCACHE_BUCKET: rust-lang-ci-sccache2
---
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM0cMowCzPY6
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
   Compiling profiler_builtins v0.0.0 (/Users/runner/work/rust/rust/library/profiler_builtins)
   Compiling alloc v0.0.0 (/Users/runner/work/rust/rust/library/alloc)
The following warnings were emitted during compilation:

warning: ../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:24:10: fatal error: 'errno.h' file not found
warning: #include <errno.h>
warning: 1 error generated.

error: failed to run custom build command for `profiler_builtins v0.0.0 (/Users/runner/work/rust/rust/library/profiler_builtins)`


Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/release/build/profiler_builtins-a4b31b78e55f61bb/build-script-build` (exit status: 1)
  --- stdout
  TARGET = Some("x86_64-apple-darwin")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-apple-darwin")
  cargo:rerun-if-env-changed=CC_x86_64-apple-darwin
  CC_x86_64-apple-darwin = None
  cargo:rerun-if-env-changed=CC_x86_64_apple_darwin
  CC_x86_64_apple_darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_apple_darwin
  CFLAGS_x86_64_apple_darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/a1bebd5c848bfc2516cda59be6bb55b320c310d9")
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-gdwarf-2" "-fno-omit-frame-pointer" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/a1bebd5c848bfc2516cda59be6bb55b320c310d9" "-I" "../../src/llvm-project/compiler-rt/include" "-fno-builtin" "-fomit-frame-pointer" "-fvisibility=hidden" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-DCOMPILER_RT_HAS_ATOMICS=1" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-dae79822b045e16a/out/../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.o" "-c" "../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c"
  cargo:warning=../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:24:10: fatal error: 'errno.h' file not found
  cargo:warning=#include <errno.h>
  cargo:warning=         ^~~~~~~~~
  exit status: 1

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-gdwarf-2" "-fno-omit-frame-pointer" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/a1bebd5c848bfc2516cda59be6bb55b320c310d9" "-I" "../../src/llvm-project/compiler-rt/include" "-fno-builtin" "-fomit-frame-pointer" "-fvisibility=hidden" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-DCOMPILER_RT_HAS_ATOMICS=1" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-dae79822b045e16a/out/../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.o" "-c" "../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c" with args "clang" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:06:39
