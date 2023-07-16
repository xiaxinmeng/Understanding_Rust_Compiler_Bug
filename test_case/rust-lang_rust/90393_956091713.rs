
...
Building stage0 compiler artifacts (x86_64-unknown-illumos -> x86_64-unknown-illumos)
   Compiling rustc_driver v0.0.0 (/nobackup/SFW_build/rust/compiler/rustc_driver)
error: linking with `gcc` failed: exit status: 1
...
  = note: Text relocation remains                 referenced
              against symbol    offsetin file
          vtable for llvm::format_object<float> 0x62      /nobackup/SFW_build/rust/build/x86_64-unknown-illumos/stage0-rustc/x86_64-unknown-illumos/release/deps/librustc_llvm-b3854e7bc8fbee87.rlib(AArch64I
nstPrinter.cpp.o)
          vtable for llvm::format_object<float> 0x5d      /nobackup/SFW_build/rust/build/x86_64-unknown-illumos/stage0-rustc/x86_64-unknown-illumos/release/deps/librustc_llvm-b3854e7bc8fbee87.rlib(AArch64InstPrinter.cpp.o)
...
