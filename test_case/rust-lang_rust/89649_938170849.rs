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
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 1767:
     fn check_macro_export(&self, hir_id: HirId, attr: &Attribute, target: Target) {
         if target != Target::MacroDef {
             self.tcx.struct_span_lint_hir(UNUSED_ATTRIBUTES, hir_id, attr.span, |lint| {
-                lint.build("`#[macro_export]` only has an effect on macro definitions")
-                    .emit();
+                lint.build("`#[macro_export]` only has an effect on macro definitions").emit();
             });
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/upvars.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_passes/src/entry.rs" "/checkout/compiler/rustc_passes/src/lib.rs" "/checkout/compiler/rustc_passes/src/hir_id_validator.rs" "/checkout/compiler/rustc_passes/src/naked_functions.rs" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
