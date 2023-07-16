plain
[01:24:38] ---- [assembly] assembly/nvptx-arch-default.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-arch-default.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-default/nvptx-arch-default.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-default/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-default/nvptx-arch-default.nvptx_arch_default.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-default/nvptx-arch-default.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-default/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-arch-link-arg.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-arch-link-arg.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-link-arg/nvptx-arch-link-arg.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-C" "link-arg=--arch=sm_60" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-link-arg/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-link-arg/nvptx-arch-link-arg.nvptx_arch_link_arg.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-link-arg/nvptx-arch-link-arg.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-link-arg/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-link-arg/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--arch=sm_60" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-arch-target-cpu.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-arch-target-cpu.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-target-cpu/nvptx-arch-target-cpu.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-C" "target-cpu=sm_50" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-target-cpu/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-target-cpu/nvptx-arch-target-cpu.nvptx_arch_target_cpu.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-target-cpu/nvptx-arch-target-cpu.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-target-cpu/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-arch-target-cpu/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_50"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-atomics.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-atomics.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-atomics/nvptx-atomics.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-atomics/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-atomics/nvptx-atomics.nvptx_atomics.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-atomics/nvptx-atomics.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-atomics/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-atomics/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-internalizing.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-internalizing.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/nvptx-internalizing.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/nvptx-internalizing.nvptx_internalizing.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/nvptx-internalizing.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/auxiliary/libnon_inline_dependency.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-internalizing/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-linking-binary.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-linking-binary.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/nvptx-linking-binary.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "bin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/nvptx-linking-binary.nvptx_linking_binary.7rcbfp3g-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/nvptx-linking-binary.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/auxiliary/libnon_inline_dependency.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-binary/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-safe-naming.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-safe-naming.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/nvptx-safe-naming.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/nvptx-safe-naming.nvptx_safe_naming.3a1fbbbh-cgu.0.rcgu.o" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/nvptx-safe-naming.nvptx_safe_naming.3a1fbbbh-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/nvptx-safe-naming.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-safe-naming/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] ---- [assembly] assembly/nvptx-linking-cdylib.rs stdout ----
[01:24:38] 
[01:24:38] error: compilation failed!
[01:24:38] status: exit code: 1
[01:24:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/nvptx-linking-cdylib.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/nvptx-linking-cdylib.s" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--crate-type" "cdylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/auxiliary"
[01:24:38] ------------------------------------------
[01:24:38] 
[01:24:38] ------------------------------------------
[01:24:38] stderr:
[01:24:38] stderr:
[01:24:38] ------------------------------------------
[01:24:38] error: linking with `rust-ptx-linker` failed: signal: 4
[01:24:38]   |
[01:24:38]   = note: "rust-ptx-linker" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--bitcode" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/nvptx-linking-cdylib.nvptx_linking_cdylib.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/nvptx-linking-cdylib.s" "-Olto" "--debug" "-L" "/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/auxiliary/libnon_inline_dependency.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/nvptx-linking-cdylib/auxiliary/libbreakpoint_panic_handler.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/librustc_std_workspace_core-032cf64734068a69.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcore-dc02be4370f738cf.rlib" "--rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib/libcompiler_builtins-c0e278bc3f061ce0.rlib" "--fallback-arch" "sm_30"
[01:24:38]   = note: [2019-04-16T02:32:54Z WARN  ptx_linker::session] The output extension is not '.ptx'. Consider changing from '.s' to '.ptx'
[01:24:38]           Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
[01:24:38]           thread 'main' panicked at 'explicit panic', /home/den/rust-ptx-linker/target/release/build/rustc-llvm-proxy-bca7ccf705470dd5/out/llvm_gen.rs:4:1
[01:24:38]           
[01:24:38] 
[01:24:38] error: aborting due to previous error
[01:24:38] 
---
[01:24:38] test result: FAILED. 1 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out
[01:24:38] 
[01:24:38] 
[01:24:38] 
[01:24:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-nvptx64-nvidia-cuda" "--mode" "assembly" "--target" "nvptx64-nvidia-cuda" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:24:38] expected success, got: exit code: 101
[01:24:38] 
[01:24:38] 
[01:24:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target nvptx64-nvidia-cuda src/test/run-make src/test/assembly
---
travis_time:end:1eae810c:start=1555381978025977880,finish=1555381978043705316,duration=17727436
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19e692ee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:102ebbc0
travis_time:start:102ebbc0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b99bd12
$ dmesg | grep -i kill
