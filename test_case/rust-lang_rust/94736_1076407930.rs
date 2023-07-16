plain
   Compiling rustc-demangle v0.1.21
error: unexpected `cfg` condition value
  --> library/unwind/src/libunwind.rs:37:30
   |
37 |     any(target_arch = "arm", target_arch = "arm64_32"),
   |
   |
   = note: `-D unexpected-cfgs` implied by `-D warnings`
   = note: expected values for `target_arch` are: aarch64, arm, asmjs, avr, bpf, hexagon, le32, m68k, mips, mips64, msp430, nvptx, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, spirv, wasm32, wasm64, x86, x86_64, xtensa
error: could not compile `unwind` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:11
