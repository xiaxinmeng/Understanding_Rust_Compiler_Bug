plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/asm/mod.rs at line 353:
             Self::RiscV(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
             Self::PowerPC(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
             Self::Hexagon(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
-            Self::LoongArch(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
+            Self::LoongArch(r) => {
+                r.validate(arch, reloc_model, target_features, target, is_clobber)
+            }
             Self::Mips(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
             Self::S390x(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
             Self::Bpf(r) => r.validate(arch, reloc_model, target_features, target, is_clobber),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/weak_lang_items.rs" "/checkout/compiler/rustc_codegen_llvm/src/va_arg.rs" "/checkout/compiler/rustc_codegen_llvm/src/base.rs" "/checkout/compiler/rustc_target/src/asm/wasm.rs" "/checkout/compiler/rustc_target/src/asm/arm.rs" "/checkout/compiler/rustc_target/src/asm/mips.rs" "/checkout/compiler/rustc_target/src/asm/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/cpp_like.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
