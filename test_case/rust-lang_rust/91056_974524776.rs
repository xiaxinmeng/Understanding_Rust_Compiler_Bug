plain
 ---> c8dcce624f64
Step 7/47 : RUN add-apt-repository -y 'deb https://apt.dilos.org/dilos dilos2 main'
 ---> Using cache
 ---> 7c6e019d29ee
Step 8/47 : ENV     AR_x86_64_unknown_fuchsia=x86_64-unknown-fuchsia-ar     CC_x86_64_unknown_fuchsia=x86_64-unknown-fuchsia-clang     CXX_x86_64_unknown_fuchsia=x86_64-unknown-fuchsia-clang++     AR_aarch64_unknown_fuchsia=aarch64-unknown-fuchsia-ar     CC_aarch64_unknown_fuchsia=aarch64-unknown-fuchsia-clang     CXX_aarch64_unknown_fuchsia=aarch64-unknown-fuchsia-clang++     AR_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-ar     CC_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-gcc     CXX_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-g++     AR_x86_64_pc_solaris=x86_64-pc-solaris2.10-ar     CC_x86_64_pc_solaris=x86_64-pc-solaris2.10-gcc     CXX_x86_64_pc_solaris=x86_64-pc-solaris2.10-g++     AR_x86_64_sun_solaris=x86_64-sun-solaris2.10-ar     CC_x86_64_sun_solaris=x86_64-sun-solaris2.10-gcc     CXX_x86_64_sun_solaris=x86_64-sun-solaris2.10-g++     CC_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-gcc-8     CXX_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-g++-8     AR_x86_64_fortanix_unknown_sgx=ar     CC_x86_64_fortanix_unknown_sgx=clang-11     CFLAGS_x86_64_fortanix_unknown_sgx="-D__ELF__ -isystem/usr/include/x86_64-linux-gnu -mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     CXX_x86_64_fortanix_unknown_sgx=clang++-11     CXXFLAGS_x86_64_fortanix_unknown_sgx="-D__ELF__ -isystem/usr/include/x86_64-linux-gnu -mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     AR_i686_unknown_freebsd=i686-unknown-freebsd11-ar     CC_i686_unknown_freebsd=i686-unknown-freebsd11-clang     CXX_i686_unknown_freebsd=i686-unknown-freebsd11-clang++     CC=gcc-8     CXX=g++-8
 ---> b1759dd63cc1
Step 9/47 : WORKDIR /build
 ---> Using cache
 ---> e515b8e69372
---
 ---> 78b52ee76ab7
Step 28/47 : ENV CARGO_TARGET_X86_64_UNKNOWN_FUCHSIA_AR /usr/local/bin/llvm-ar
 ---> Using cache
 ---> 8606348cc597
Step 29/47 : ENV CARGO_TARGET_X86_64_UNKNOWN_FUCHSIA_RUSTFLAGS -C link-arg=--sysroot=/usr/local/x86_64-unknown-fuchsia -C link-arg=-L/usr/local/x86_64-unknown-fuchsia/lib -C link-arg=-L/usr/local/lib/x86_64-unknown-fuchsia/lib
 ---> 739861f06989
Step 30/47 : ENV CARGO_TARGET_AARCH64_UNKNOWN_FUCHSIA_AR /usr/local/bin/llvm-ar
 ---> Using cache
 ---> 3e241d7ae409
 ---> 3e241d7ae409
Step 31/47 : ENV CARGO_TARGET_AARCH64_UNKNOWN_FUCHSIA_RUSTFLAGS -C link-arg=--sysroot=/usr/local/aarch64-unknown-fuchsia -C link-arg=-L/usr/local/aarch64-unknown-fuchsia/lib -C link-arg=-L/usr/local/lib/aarch64-unknown-fuchsia/lib
 ---> 0d1567b5e91c
Step 32/47 : ENV TARGETS=x86_64-unknown-fuchsia
 ---> Using cache
 ---> 89969d53edda
---
[RUSTC-TIMING] addr2line test:false 0.503
[RUSTC-TIMING] core test:false 28.414
[RUSTC-TIMING] gimli test:false 5.433
[RUSTC-TIMING] object test:false 5.604
error: linking with `rust-lld` failed: exit status: 1
  |
  = note: "rust-lld" "-flavor" "gnu" "--version-script=/tmp/rustcVJCpuG/list" "--build-id" "--hash-style=gnu" "-z" "max-page-size=4096" "-z" "now" "-z" "rodynamic" "-z" "separate-loadable-segments" "--pack-dyn-relocs=relr" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/std-885c4f3479434cef.std.5bc82035-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/std-885c4f3479434cef.7omemo2aus3kgps.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/std-885c4f3479434cef.2lz4movwa8gaxi5c.rcgu.o" "--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/build/compiler_builtins-6a60ae20bd93e083/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-fuchsia/lib" "-lzircon" "-lfdio" "-Bstatic" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_unwind-2161bcc1054373c7.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libminiz_oxide-f093a76a7b47e609.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libadler-23167bcf207c6d7d.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libobject-b94a27d0a39135bf.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libmemchr-621cd1f435dba2ee.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libaddr2line-941e43b0b939d99d.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libgimli-99d93059021e39a9.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libstd_detect-9c6974a529ba013d.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/librustc_demangle-034c1b743df10eb7.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libhashbrown-d35431155d368d7a.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/librustc_std_workspace_alloc-0f9bdba3664dd4dc.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libunwind-2ec961f47a1e3308.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcfg_if-c37f6fbf3c99d01d.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liblibc-9d23694209a8a939.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc-df761ad9d176da1e.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/librustc_std_workspace_core-fb32395deb7b4ac1.rlib" "--no-whole-archive" "--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcore-27342868d30aef11.rlib" "--no-whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcompiler_builtins-87575e986b495b8c.rlib" "-Bdynamic" "-lunwind" "-lc" "-lfdio" "--eh-frame-hdr" "-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-fuchsia/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libstd-885c4f3479434cef.so" "-shared" "-O1" "--sysroot=/usr/local/x86_64-unknown-fuchsia" "-L/usr/local/x86_64-unknown-fuchsia/lib" "-L/usr/local/lib/x86_64-unknown-fuchsia/lib"
  = note: rust-lld: error: unable to find library -lunwind

[RUSTC-TIMING] std test:false 26.808
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:13:57
