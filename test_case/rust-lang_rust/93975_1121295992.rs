
error: unknown directive
  --> /home/buildozer/aports/community/rust/src/rustc-1.60.0-src/library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:11:20
   |
11 |     unsafe { asm!(".insn i 0x0F, 0, x0, x0, 0x010", options(nomem, nostack)) }
   |                    ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
1  |     .insn i 0x0F, 0, x0, x0, 0x010
   |     ^

   Compiling aho-corasick v0.7.18
error: could not compile `log` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:17:42
