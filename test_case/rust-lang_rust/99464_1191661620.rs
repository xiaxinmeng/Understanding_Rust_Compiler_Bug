
  running: "sccache" "aarch64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=aarch64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=aarch64-fuchsia" "-I" "/checkout/src/llvm-project/compiler-rt/lib/builtins" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-fuchsia/release/build/compiler_builtins-997f5fa9cc5c4633/out/cpu_model.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cpu_model.c"
  cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/cpu_model.c:812:10: fatal error: 'zircon/features.h' file not found
  cargo:warning=#include <zircon/features.h>
  cargo:warning=         ^~~~~~~~~~~~~~~~~~~
  cargo:warning=1 error generated.
  exit status: 1

  --- stderr


  error occurred: Command "sccache" "aarch64-fuchsia-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=aarch64-fuchsia" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=aarch64-fuchsia" "-I" "/checkout/src/llvm-project/compiler-rt/lib/builtins" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-fuchsia/release/build/compiler_builtins-997f5fa9cc5c4633/out/cpu_model.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cpu_model.c" with args "aarch64-fuchsia-clang" did not execute successfully (status code exit status: 1).
