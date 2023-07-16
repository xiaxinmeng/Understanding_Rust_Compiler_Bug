plain
[RUSTC-TIMING] build_script_build test:false 0.593
error: failed to run custom build command for `compiler_builtins v0.1.44`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-77086286f956e553/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.44/compiler-rt
  cargo:rustc-cfg=feature="unstable"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c
  cargo:rustc-cfg=__absvdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c
  cargo:rustc-cfg=__absvsi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c
  cargo:rustc-cfg=__absvti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c
  cargo:rustc-cfg=__addvdi3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c
  cargo:rustc-cfg=__addvsi3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c
  cargo:rustc-cfg=__addvti3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzdi2.c
  cargo:rustc-cfg=__clzdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzsi2.c
  cargo:rustc-cfg=__clzsi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c
  cargo:rustc-cfg=__clzti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c
  cargo:rustc-cfg=__cmpdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c
  cargo:rustc-cfg=__cmpti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c
  cargo:rustc-cfg=__ctzdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c
  cargo:rustc-cfg=__ctzsi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c
  cargo:rustc-cfg=__ctzti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c
  cargo:rustc-cfg=__divdc3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c
  cargo:rustc-cfg=__divsc3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c
  cargo:rustc-cfg=__divxc3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c
  cargo:rustc-cfg=__extendhfsf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c
  cargo:rustc-cfg=__ffsti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdisf.c
  cargo:rustc-cfg=__floatdisf="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdixf.c
  cargo:rustc-cfg=__floatdixf="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundidf.S
  cargo:rustc-cfg=__floatundidf="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundisf.S
  cargo:rustc-cfg=__floatundisf="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundixf.S
  cargo:rustc-cfg=__floatundixf="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/int_util.c
  cargo:rustc-cfg=__int_util="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/muldc3.c
  cargo:rustc-cfg=__muldc3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c
  cargo:rustc-cfg=__mulsc3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c
  cargo:rustc-cfg=__mulvdi3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c
  cargo:rustc-cfg=__mulvsi3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c
  cargo:rustc-cfg=__mulvti3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c
  cargo:rustc-cfg=__mulxc3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdf2.c
  cargo:rustc-cfg=__negdf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdi2.c
  cargo:rustc-cfg=__negdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negsf2.c
  cargo:rustc-cfg=__negsf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negti2.c
  cargo:rustc-cfg=__negti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c
  cargo:rustc-cfg=__negvdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c
  cargo:rustc-cfg=__negvsi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvti2.c
  cargo:rustc-cfg=__negvti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c
  cargo:rustc-cfg=__paritydi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c
  cargo:rustc-cfg=__paritysi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/parityti2.c
  cargo:rustc-cfg=__parityti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c
  cargo:rustc-cfg=__popcountdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c
  cargo:rustc-cfg=__popcountsi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c
  cargo:rustc-cfg=__popcountti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/powixf2.c
  cargo:rustc-cfg=__powixf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c
  cargo:rustc-cfg=__subvdi3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c
  cargo:rustc-cfg=__subvsi3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvti3.c
  cargo:rustc-cfg=__subvti3="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfhf2.c
  cargo:rustc-cfg=__truncdfhf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfsf2.c
  cargo:rustc-cfg=__truncdfsf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c
  cargo:rustc-cfg=__truncsfhf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c
  cargo:rustc-cfg=__ucmpdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c
  cargo:rustc-cfg=__ucmpti2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/apple_versioning.c
  cargo:rustc-cfg=apple_versioning="optimized-c"
  TARGET = Some("x86_64-fuchsia")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-unknown-linux-gnu")
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia")
  CRATE_CC_NO_DEFAULTS = None
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia")
  CRATE_CC_NO_DEFAULTS = None
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/absvsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/absvti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/addvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/addvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/addvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/clzdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/clzdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/clzsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/clzsi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/clzti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/cmpdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/cmpti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ctzdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ctzsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ctzti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/divdc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/divsc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/divxc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/extendhfsf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ffsti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatdisf.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdisf.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatdixf.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdixf.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatundidf.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundidf.S"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatundisf.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundisf.S"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatundixf.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatundixf.S"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/int_util.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/int_util.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/muldc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/muldc3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulsc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulxc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negdf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negdf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negsf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negsf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negvsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negvti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negvti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/paritydi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/paritysi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/parityti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/parityti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/popcountdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/popcountsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/popcountti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/powixf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/powixf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/subvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/subvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/subvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/subvti3.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/truncdfhf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfhf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/truncdfsf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfsf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/truncsfhf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ucmpdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ucmpti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c"
  exit status: 0
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/apple_versioning.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/apple_versioning.c"
  exit status: 0
  AR_x86_64-fuchsia = Some("x86_64-fuchsia-ar")
  running: "x86_64-fuchsia-ar" "cq" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/libcompiler-rt.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/absvdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/absvsi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/absvti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/addvdi3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/addvsi3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/addvti3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/clzdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/clzsi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/clzti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/cmpdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/cmpti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ctzdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ctzsi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ctzti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/divdc3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/divsc3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/divxc3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/extendhfsf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ffsti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatdisf.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatdixf.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatundidf.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatundisf.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/floatundixf.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/int_util.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/muldc3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulsc3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulvdi3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulvsi3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulvti3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/mulxc3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negdf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negsf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negvdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negvsi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/negvti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/paritydi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/paritysi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/parityti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/popcountdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/popcountsi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/popcountti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/powixf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/subvdi3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/subvsi3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/subvti3.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/truncdfhf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/truncdfsf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/truncsfhf2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ucmpdi2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/ucmpti2.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/apple_versioning.o"
  running: "x86_64-fuchsia-ar" "s" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out/libcompiler-rt.a"
  exit status: 0
  exit status: 0
  cargo:rustc-link-lib=static=compiler-rt
  cargo:rustc-link-search=native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-f0db73d0d72ba66b/out
  --- stderr
  thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2', /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.44/build.rs:75:69
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
