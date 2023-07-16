plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 453:
                     let e_flags;
                     if sess.target.options.cpu.contains("r6") {
                         // copied from `mipsisa32r6el-linux-gnu-gcc foo.c -c` generated .o file `e_flags` field
-                        e_flags = elf::EF_MIPS_ARCH_32R6 | elf::EF_MIPS_NAN2008 | elf::EF_MIPS_CPIC | elf::EF_MIPS_PIC;
+                        e_flags = elf::EF_MIPS_ARCH_32R6
+                            | elf::EF_MIPS_NAN2008
+                            | elf::EF_MIPS_CPIC
+                            | elf::EF_MIPS_PIC;
                     } else {
                         e_flags = elf::EF_MIPS_ARCH_32R2 | elf::EF_MIPS_CPIC | elf::EF_MIPS_PIC;
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 464:
                     let e_flags;
                     let e_flags;
                     if sess.target.options.cpu.contains("r6") {
                         // copied from `mipsisa64r6el-linux-gnuabi64-gcc foo.c -c` generated .o file `e_flags` field
-                        e_flags = elf::EF_MIPS_ARCH_64R6 | elf::EF_MIPS_NAN2008 | elf::EF_MIPS_CPIC | elf::EF_MIPS_PIC;
+                        e_flags = elf::EF_MIPS_ARCH_64R6
+                            | elf::EF_MIPS_NAN2008
+                            | elf::EF_MIPS_CPIC
+                            | elf::EF_MIPS_PIC;
                     } else {
                         e_flags = elf::EF_MIPS_ARCH_64R2 | elf::EF_MIPS_CPIC | elf::EF_MIPS_PIC;
                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/command.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/lto.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/debuginfo.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
