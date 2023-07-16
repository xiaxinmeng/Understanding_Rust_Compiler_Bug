plain
 ---> ea77e1d93ab7
Step 6/48 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
 ---> Running in b906b8b46254
Warning: apt-key output should not be parsed (stdout is not a terminal)
Executing: /tmp/apt-key-gpghome.R8th2kCymc/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
gpg: Total number processed: 1
gpg:               imported: 1
Removing intermediate container b906b8b46254
 ---> 92ce9a30ecb6
---
Fetched 22.5 kB in 1s (15.5 kB/s)
Reading package lists...
Removing intermediate container a0f8f47404a3
 ---> d4e4a8fce981
Step 8/48 : ENV     AR_x86_64_fuchsia=x86_64-fuchsia-ar     CC_x86_64_fuchsia=x86_64-fuchsia-clang     CFLAGS_x86_64_fuchsia="--target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot"     CXX_x86_64_fuchsia=x86_64-fuchsia-clang++     CXXFLAGS_x86_64_fuchsia="--target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot"     LDFLAGS_x86_64_fuchsia="--target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -L/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/lib"     AR_aarch64_fuchsia=aarch64-fuchsia-ar     CC_aarch64_fuchsia=aarch64-fuchsia-clang     CFLAGS_aarch64_fuchsia="--target=aarch64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot"     CXX_aarch64_fuchsia=aarch64-fuchsia-clang++     CXXFLAGS_aarch64_fuchsia="--target=aarch64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot"     LDFLAGS_aarch64_fuchsia="--target=aarch64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot -L/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/lib"     AR_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-ar     CC_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-gcc     CXX_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-g++     AR_x86_64_pc_solaris=x86_64-pc-solaris2.10-ar     CC_x86_64_pc_solaris=x86_64-pc-solaris2.10-gcc     CXX_x86_64_pc_solaris=x86_64-pc-solaris2.10-g++     AR_x86_64_sun_solaris=x86_64-sun-solaris2.10-ar     CC_x86_64_sun_solaris=x86_64-sun-solaris2.10-gcc     CXX_x86_64_sun_solaris=x86_64-sun-solaris2.10-g++     CC_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-gcc-8     CXX_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-g++-8     AR_x86_64_fortanix_unknown_sgx=ar     CC_x86_64_fortanix_unknown_sgx=clang-11     CFLAGS_x86_64_fortanix_unknown_sgx="-D__ELF__ -isystem/usr/include/x86_64-linux-gnu -mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     CXX_x86_64_fortanix_unknown_sgx=clang++-11     CXXFLAGS_x86_64_fortanix_unknown_sgx="-D__ELF__ -isystem/usr/include/x86_64-linux-gnu -mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     AR_i686_unknown_freebsd=i686-unknown-freebsd12-ar     CC_i686_unknown_freebsd=i686-unknown-freebsd12-clang     CXX_i686_unknown_freebsd=i686-unknown-freebsd12-clang++     CC=gcc-8     CXX=g++-8
Removing intermediate container da31776db483
 ---> 878b8039e47e
Step 9/48 : WORKDIR /build
 ---> Running in 3d90dbdae06e
---
 ---> e77958b64330
Step 15/48 : RUN /tmp/build-fuchsia-toolchain.sh
 ---> Running in 59595c53ce9e
+ source shared.sh
+ FUCHSIA_SDK_URL=https://chrome-infra-packages.appspot.com/dl/fuchsia/sdk/core/linux-amd64
+ FUCHSIA_SDK_ID=4xjxrGUrDbQ6_zJwj6cDN1IbWsWV5aCQXC_zO_Hu0XkC
+ FUCHSIA_SDK_SHA256=e318f1ac652b0db43aff32708fa70337521b5ac595e5a0905c2ff33bf1eed179
+ FUCHSIA_SDK_USR_DIR=/usr/local/core-linux-amd64-fuchsia-sdk
+ CLANG_DOWNLOAD_URL=https://chrome-infra-packages.appspot.com/dl/fuchsia/third_party/clang/linux-amd64
+ CLANG_DOWNLOAD_ID=vU0vNjSihOV4Q6taQYCpy03JXGiCyVwxen3rFMNMIgsC
+ CLANG_DOWNLOAD_SHA256=bd4d2f3634a284e57843ab5a4180a9cb4dc95c6882c95c317a7deb14c34c220b
+ hide_output install_clang
+ hide_output install_zircon_libs
Removing intermediate container 59595c53ce9e
 ---> f70b2506dc1f
Step 16/48 : COPY host-x86_64/dist-various-2/build-solaris-toolchain.sh /tmp/
 ---> 81dc11b81b4b
---
Step 28/48 : ENV CARGO_TARGET_X86_64_FUCHSIA_AR /usr/local/bin/llvm-ar
 ---> Running in 4e40d9ca8772
Removing intermediate container 4e40d9ca8772
 ---> 03ccd1ae9bc9
Step 29/48 : ENV CARGO_TARGET_X86_64_FUCHSIA_RUSTFLAGS -C link-arg=--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot/lib -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/lib
Removing intermediate container 37202a7a6e09
 ---> 8c060dd91ff4
Step 30/48 : ENV CARGO_TARGET_AARCH64_FUCHSIA_AR /usr/local/bin/llvm-ar
 ---> Running in f879ef1d07e9
 ---> Running in f879ef1d07e9
Removing intermediate container f879ef1d07e9
 ---> 678e2b73040f
Step 31/48 : ENV CARGO_TARGET_AARCH64_FUCHSIA_RUSTFLAGS -C link-arg=--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot/lib -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/lib
Removing intermediate container b6cf92f702bc
 ---> d167d7fe74f3
Step 32/48 : ENV TARGETS=x86_64-fuchsia
 ---> Running in 8e9a8be465b2
---
[RUSTC-TIMING] build_script_build test:false 0.315
[RUSTC-TIMING] build_script_build test:false 0.615
The following warnings were emitted during compilation:

warning: sccache: error: failed to execute compile
warning: sccache: caused by: cannot find binary path
error: failed to run custom build command for `compiler_builtins v0.1.73`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-e084322c1f407057/build-script-build` (exit status: 1)
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-e084322c1f407057/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.73/compiler-rt
  cargo:rustc-cfg=feature="unstable"
  cargo:rustc-cfg=feature="mem-unaligned"
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
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/x86_64/floatdixf.c
  cargo:rustc-cfg=__floatdixf="optimized-c"
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
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c
  cargo:rustc-cfg=__truncsfhf2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c
  cargo:rustc-cfg=__ucmpdi2="optimized-c"
  cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c
  cargo:rustc-cfg=__ucmpti2="optimized-c"
  TARGET = Some("x86_64-fuchsia")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-unknown-linux-gnu")
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia --target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia --target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot")
  CRATE_CC_NO_DEFAULTS = None
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia --target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot")
  CRATE_CC_NO_DEFAULTS = None
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "--target=x86_64-fuchsia" "--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-ebbca5a578951bf7/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
  cargo:warning=sccache: error: failed to execute compile
  cargo:warning=sccache: caused by: cannot find binary path

  --- stderr



  error occurred: Command "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "--target=x86_64-fuchsia" "--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-ebbca5a578951bf7/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c" with args "x86_64-fuchsia-clang" did not execute successfully (status code exit status: 2).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 29.729
Build completed unsuccessfully in 0:13:03
