plain
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
[RUSTC-TIMING] rustc_driver test:false 8.113
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
[RUSTC-TIMING] rustc_smir test:false 0.070
error: linking with `clang` failed: exit status: 1
  |
  = note: "clang" "-m64" "/tmp/rustc3QLEN1/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-54471440a93ef821.rustc_main.a4748a89-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-54471440a93ef821.rustc_main.a4748a89-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-54471440a93ef821.rustc_main.a4748a89-cgu.2.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-69a3209389858594/out/build/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-49791a0c2372c437/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-98ca7780e475a128/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib" "-L" "/rustroot/bin/../lib/gcc/x86_64-pc-linux-gnu/7.5.0/../../../../lib64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-9b1cd69a927aa865" "-lLLVM-15-rust-1.66.0-nightly" "-ldl" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjemalloc_sys-f8a71c72a88bc04e.rlib" "/tmp/rustc3QLEN1/libjemalloc_pic.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libprofiler_builtins-1dc740fd680a189b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-3dd5f78c3b43746e" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-5b83a1df856cf582.rlib" "/tmp/rustc3QLEN1/libcompiler-rt.a" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-54471440a93ef821" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-u" "__llvm_profile_runtime" "-Wl,-z,origin" "-Wl,-rpath,$ORIGIN/../lib" "-fuse-ld=lld" "-Wl,--icf=all"
  = note: ld.lld: warning: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjemalloc_sys-f8a71c72a88bc04e.rlib: archive member 'libjemalloc_pic.a' is neither ET_REL nor LLVM bitcode
          ld.lld: warning: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libprofiler_builtins-1dc740fd680a189b.rlib: archive member 'libprofiler-rt.a' is neither ET_REL nor LLVM bitcode
          ld.lld: warning: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-5b83a1df856cf582.rlib: archive member 'libcompiler-rt.a' is neither ET_REL nor LLVM bitcode
          ld.lld: error: undefined symbol: __llvm_profile_instrument_target
          >>> referenced by rustc_main.a4748a89-cgu.2
          >>>               /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-54471440a93ef821.rustc_main.a4748a89-cgu.2.rcgu.o:(std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>)
          
          ld.lld: error: undefined reference due to --no-allow-shlib-undefined: __llvm_profile_instrument_memop
          >>> referenced by /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-9b1cd69a927aa865.so
          
          ld.lld: error: undefined reference due to --no-allow-shlib-undefined: __llvm_profile_instrument_target
          >>> referenced by /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-9b1cd69a927aa865.so
          clang-15: error: linker command failed with exit code 1 (use -v to see invocation)

[RUSTC-TIMING] rustc_main test:false 0.269
error: could not compile `rustc-main` due to previous error
Build completed unsuccessfully in 0:07:41
