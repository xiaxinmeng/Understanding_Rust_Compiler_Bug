plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 748cb1f01d623f2afd0d8b84fda7e2c8f7a11c7b and 53748608ee83bbda8693065baccf862bc38ae723
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling rustc-demangle v0.1.21
error: unexpected `cfg` condition value
  --> library/unwind/src/libunwind.rs:72:7
   |
72 | #[cfg(target_arch = "loongarch64")]
   |
   |
   = note: `-D unexpected-cfgs` implied by `-D warnings`
   = note: expected values for `target_arch` are: aarch64, arm, asmjs, avr, bpf, hexagon, le32, m68k, mips, mips64, msp430, nvptx, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, spirv, wasm32, wasm64, x86, x86_64, xtensa
error: could not compile `unwind` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:16
cat: /tmp/toolstate/toolstates.json: No such file or directory
