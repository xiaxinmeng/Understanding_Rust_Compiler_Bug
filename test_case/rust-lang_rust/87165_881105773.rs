plain

error: failed to run custom build command for `compiler_builtins v0.1.47`

Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/release/build/compiler_builtins-6ce9bfb94d793bee/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:compiler-rt=/Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.47/compiler-rt
  cargo:rustc-cfg=feature="unstable"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c
  cargo:rustc-cfg=__absvdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c
  cargo:rustc-cfg=__absvsi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/absvti2.c
  cargo:rustc-cfg=__absvti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c
  cargo:rustc-cfg=__addvdi3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c
  cargo:rustc-cfg=__addvsi3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/addvti3.c
  cargo:rustc-cfg=__addvti3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/clzdi2.c
  cargo:rustc-cfg=__clzdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/clzsi2.c
  cargo:rustc-cfg=__clzsi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/clzti2.c
  cargo:rustc-cfg=__clzti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c
  cargo:rustc-cfg=__cmpdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c
  cargo:rustc-cfg=__cmpti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c
  cargo:rustc-cfg=__ctzdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c
  cargo:rustc-cfg=__ctzsi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c
  cargo:rustc-cfg=__ctzti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/divdc3.c
  cargo:rustc-cfg=__divdc3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/divsc3.c
  cargo:rustc-cfg=__divsc3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/divxc3.c
  cargo:rustc-cfg=__divxc3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c
  cargo:rustc-cfg=__extendhfsf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c
  cargo:rustc-cfg=__ffsti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdisf.c
  cargo:rustc-cfg=__floatdisf="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdixf.c
  cargo:rustc-cfg=__floatdixf="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundidf.S
  cargo:rustc-cfg=__floatundidf="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundisf.S
  cargo:rustc-cfg=__floatundisf="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundixf.S
  cargo:rustc-cfg=__floatundixf="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/int_util.c
  cargo:rustc-cfg=__int_util="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/muldc3.c
  cargo:rustc-cfg=__muldc3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c
  cargo:rustc-cfg=__mulsc3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c
  cargo:rustc-cfg=__mulvdi3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c
  cargo:rustc-cfg=__mulvsi3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c
  cargo:rustc-cfg=__mulvti3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c
  cargo:rustc-cfg=__mulxc3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negdf2.c
  cargo:rustc-cfg=__negdf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negdi2.c
  cargo:rustc-cfg=__negdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negsf2.c
  cargo:rustc-cfg=__negsf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negti2.c
  cargo:rustc-cfg=__negti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c
  cargo:rustc-cfg=__negvdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c
  cargo:rustc-cfg=__negvsi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/negvti2.c
  cargo:rustc-cfg=__negvti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c
  cargo:rustc-cfg=__paritydi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c
  cargo:rustc-cfg=__paritysi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/parityti2.c
  cargo:rustc-cfg=__parityti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c
  cargo:rustc-cfg=__popcountdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c
  cargo:rustc-cfg=__popcountsi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c
  cargo:rustc-cfg=__popcountti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/powixf2.c
  cargo:rustc-cfg=__powixf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c
  cargo:rustc-cfg=__subvdi3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c
  cargo:rustc-cfg=__subvsi3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/subvti3.c
  cargo:rustc-cfg=__subvti3="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/truncdfhf2.c
  cargo:rustc-cfg=__truncdfhf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/truncdfsf2.c
  cargo:rustc-cfg=__truncdfsf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c
  cargo:rustc-cfg=__truncsfhf2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c
  cargo:rustc-cfg=__ucmpdi2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c
  cargo:rustc-cfg=__ucmpti2="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/apple_versioning.c
  cargo:rustc-cfg=apple_versioning="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/atomic_flag_clear.c
  cargo:rustc-cfg=atomic_flag_clear="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/atomic_flag_clear_explicit.c
  cargo:rustc-cfg=atomic_flag_clear_explicit="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/atomic_flag_test_and_set.c
  cargo:rustc-cfg=atomic_flag_test_and_set="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/atomic_flag_test_and_set_explicit.c
  cargo:rustc-cfg=atomic_flag_test_and_set_explicit="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/atomic_signal_fence.c
  cargo:rustc-cfg=atomic_signal_fence="optimized-c"
  cargo:rerun-if-changed=/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/atomic_thread_fence.c
  cargo:rustc-cfg=atomic_thread_fence="optimized-c"
  TARGET = Some("x86_64-apple-darwin")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-apple-darwin")
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/c9f304b7f544f29832bbeb0efd5496489841eaa7")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/c9f304b7f544f29832bbeb0efd5496489841eaa7")
  CRATE_CC_NO_DEFAULTS = None
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/c9f304b7f544f29832bbeb0efd5496489841eaa7")
  CRATE_CC_NO_DEFAULTS = None
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/c9f304b7f544f29832bbeb0efd5496489841eaa7" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-d73e3d9b082b1fa7/out/absvdi2.o" "-c" "/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
  cargo:warning=error: Connection to server timed out

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/c9f304b7f544f29832bbeb0efd5496489841eaa7" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-d73e3d9b082b1fa7/out/absvdi2.o" "-c" "/Users/runner/work/rust/rust/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c" with args "clang" did not execute successfully (status code exit status: 2).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 786.366
error: build failed
