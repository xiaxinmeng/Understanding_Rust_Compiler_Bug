plain
 ---> efc16de51813
Step 6/48 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
 ---> Running in de18ed6240a4
Warning: apt-key output should not be parsed (stdout is not a terminal)
Executing: /tmp/apt-key-gpghome.kbuyy0GUXo/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
gpg: Total number processed: 1
gpg:               imported: 1
Removing intermediate container de18ed6240a4
 ---> f442418d332a
---
Fetched 22.5 kB in 1s (23.3 kB/s)
Reading package lists...
Removing intermediate container f30ad994d05f
 ---> 618507df2e6a
Step 8/48 : ENV     AR_x86_64_fuchsia=x86_64-fuchsia-ar     CC_x86_64_fuchsia=x86_64-fuchsia-clang     CFLAGS_x86_64_fuchsia="--target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include"     CXX_x86_64_fuchsia=x86_64-fuchsia-clang++     CXXFLAGS_x86_64_fuchsia="--target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include"     LDFLAGS_x86_64_fuchsia="--target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -L/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/lib"     AR_aarch64_fuchsia=aarch64-fuchsia-ar     CC_aarch64_fuchsia=aarch64-fuchsia-clang     CFLAGS_aarch64_fuchsia="--target=aarch64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include"     CXX_aarch64_fuchsia=aarch64-fuchsia-clang++     CXXFLAGS_aarch64_fuchsia="--target=aarch64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include"     LDFLAGS_aarch64_fuchsia="--target=aarch64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot -L/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/lib"     AR_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-ar     CC_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-gcc     CXX_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-g++     AR_x86_64_pc_solaris=x86_64-pc-solaris2.10-ar     CC_x86_64_pc_solaris=x86_64-pc-solaris2.10-gcc     CXX_x86_64_pc_solaris=x86_64-pc-solaris2.10-g++     AR_x86_64_sun_solaris=x86_64-sun-solaris2.10-ar     CC_x86_64_sun_solaris=x86_64-sun-solaris2.10-gcc     CXX_x86_64_sun_solaris=x86_64-sun-solaris2.10-g++     CC_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-gcc-8     CXX_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-g++-8     AR_x86_64_fortanix_unknown_sgx=ar     CC_x86_64_fortanix_unknown_sgx=clang-11     CFLAGS_x86_64_fortanix_unknown_sgx="-D__ELF__ -isystem/usr/include/x86_64-linux-gnu -mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     CXX_x86_64_fortanix_unknown_sgx=clang++-11     CXXFLAGS_x86_64_fortanix_unknown_sgx="-D__ELF__ -isystem/usr/include/x86_64-linux-gnu -mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     AR_i686_unknown_freebsd=i686-unknown-freebsd12-ar     CC_i686_unknown_freebsd=i686-unknown-freebsd12-clang     CXX_i686_unknown_freebsd=i686-unknown-freebsd12-clang++     CC=gcc-8     CXX=g++-8
Removing intermediate container 9c7ef58cadc7
 ---> c997f728e723
Step 9/48 : WORKDIR /build
 ---> Running in bcd76ecb80d7
---
 ---> 98ca430abf07
Step 15/48 : RUN /tmp/build-fuchsia-toolchain.sh
 ---> Running in 46c560ded9d1
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
 ---> 819e50ecacda
Step 16/48 : COPY host-x86_64/dist-various-2/build-solaris-toolchain.sh /tmp/
 ---> be20b67b522b
Step 17/48 : RUN /tmp/build-solaris-toolchain.sh x86_64  amd64   solaris-i386  pc
---
Step 28/48 : ENV CARGO_TARGET_X86_64_FUCHSIA_AR /usr/local/bin/llvm-ar
 ---> Running in 0ebeef21aa03
Removing intermediate container 0ebeef21aa03
 ---> 60b5bcfb35c9
Step 29/48 : ENV CARGO_TARGET_X86_64_FUCHSIA_RUSTFLAGS -C link-arg=--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot/lib -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/lib
Removing intermediate container 32d90b0a8109
 ---> 145be75e6809
Step 30/48 : ENV CARGO_TARGET_AARCH64_FUCHSIA_AR /usr/local/bin/llvm-ar
 ---> Running in 6c4acfcabc42
 ---> Running in 6c4acfcabc42
Removing intermediate container 6c4acfcabc42
 ---> 41077a4ad550
Step 31/48 : ENV CARGO_TARGET_AARCH64_FUCHSIA_RUSTFLAGS -C link-arg=--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/sysroot/lib -Lnative=/usr/local/core-linux-amd64-fuchsia-sdk/arch/arm64/lib
Removing intermediate container 3f13d8ab1d54
 ---> 5f1cbfbe2860
Step 32/48 : ENV TARGETS=x86_64-fuchsia
 ---> Running in bd59756c5998
---
[RUSTC-TIMING] build_script_build test:false 0.297
[RUSTC-TIMING] build_script_build test:false 0.528
The following warnings were emitted during compilation:

warning: In file included from /checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c:13:
warning: /checkout/src/llvm-project/compiler-rt/lib/builtins/int_lib.h:92:10: fatal error: cannot open file '/usr/local/lib/clang/16.0.0/include/float.h': Permission denied
warning: #include <float.h>
warning: 1 error generated.

error: failed to run custom build command for `compiler_builtins v0.1.73`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-c9a8b4659c88150c/build-script-build` (exit status: 1)
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
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia --target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia --target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include")
  CRATE_CC_NO_DEFAULTS = None
  CC_x86_64-fuchsia = Some("sccache x86_64-fuchsia-clang")
  CFLAGS_x86_64-fuchsia = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-fuchsia --target=x86_64-fuchsia --sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot -I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include")
  CRATE_CC_NO_DEFAULTS = None
  running: "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "--target=x86_64-fuchsia" "--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot" "-I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-d05aef61843c2f8d/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
  cargo:warning=In file included from /checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c:13:
  cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/int_lib.h:92:10: fatal error: cannot open file '/usr/local/lib/clang/16.0.0/include/float.h': Permission denied
  cargo:warning=#include <float.h>
  cargo:warning=         ^
  cargo:warning=1 error generated.

  --- stderr



  error occurred: Command "sccache" "x86_64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "--target=x86_64-fuchsia" "--sysroot=/usr/local/core-linux-amd64-fuchsia-sdk/arch/x64/sysroot" "-I/usr/local/core-linux-amd64-fuchsia-sdk/pkg/fdio/include" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-d05aef61843c2f8d/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c" with args "x86_64-fuchsia-clang" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 23.298
Build completed unsuccessfully in 0:10:22
