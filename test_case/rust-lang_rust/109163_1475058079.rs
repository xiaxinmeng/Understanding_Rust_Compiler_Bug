plain
-- LLVM_MAIN_SRC_DIR: "/checkout/src/llvm-project/llvm"
  UNSUPPORTED COMPILER-RT CONFIGURATION DETECTED: LLVM cmake package not
  found.

  Reconfigure with -DLLVM_CMAKE_DIR=/path/to/llvm.
  CMakeLists.txt:92 (load_llvm_config)


-- Attempting to mock the changes made by LLVMConfig.cmake
---
-- Could NOT find LLVM (missing: LLVM_DIR)
  UNSUPPORTED COMPILER-RT CONFIGURATION DETECTED: LLVM cmake package not
  found.

  Reconfigure with -DLLVM_CMAKE_DIR=/path/to/llvm.
  CMakeLists.txt:92 (load_llvm_config)


-- LLVM_MAIN_SRC_DIR: "/checkout/src/llvm-project/llvm"
---
  UNSUPPORTED COMPILER-RT CONFIGURATION DETECTED: LLVM cmake package not
-- Could NOT find LLVM (missing: LLVM_DIR)
  found.

  Reconfigure with -DLLVM_CMAKE_DIR=/path/to/llvm.
  CMakeLists.txt:92 (load_llvm_config)


-- LLVM_MAIN_SRC_DIR: "/checkout/src/llvm-project/llvm"
---
-- Could NOT find LLVM (missing: LLVM_DIR)
  UNSUPPORTED COMPILER-RT CONFIGURATION DETECTED: LLVM cmake package not
  found.

  Reconfigure with -DLLVM_CMAKE_DIR=/path/to/llvm.
  CMakeLists.txt:92 (load_llvm_config)


-- LLVM_MAIN_SRC_DIR: "/checkout/src/llvm-project/llvm"
---
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/1.70.0/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 254)
  --- stderr
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x97e043)[0x7f450d708043]
  /lib/x86_64-linux-gnu/libc.so.6(+0x43090)[0x7f450ca15090]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-16-rust-1.70.0-nightly.so(+0x7262e06)[0x7f450b2b2e06]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xd0fa64)[0x7f450da99a64]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbca65c)[0x7f450d95465c]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0xbc9794)[0x7f450d953794]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvXs5_Cs7DTEPoQIU0M_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsiydnvZ3aRh1_17rustc_codegen_ssa6traits7backend14CodegenBackend15target_features+0x1b)[0x7f450d9b37cb]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util17add_configuration+0x37)[0x7f450d7f5887]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(_RNvNtCsbMR7lNZiyEF_15rustc_interface4util14create_session+0x89d)[0x7f450d7f632d]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x988590)[0x7f450d712590]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x983310)[0x7f450d70d310]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-fdca23ff32bf7124.so(+0x984e53)[0x7f450d70ee53]
  /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-1c2653f0407064e2.so(rust_metadata_std_65e6c9c8149d69d4+0xca4ae)[0x7f450cc924ae]
  /lib/x86_64-linux-gnu/libpthread.so.0(+0x8609)[0x7f4503e30609]
  /lib/x86_64-linux-gnu/libc.so.6(clone+0x43)[0x7f450caf1133]
  [RUSTC-TIMING] ___ test:false 0.204
  rustc exited with signal: 11 (SIGSEGV) (core dumped)
