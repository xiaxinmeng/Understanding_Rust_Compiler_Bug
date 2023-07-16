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
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 241:
             Target::Fn
             | Target::Method(MethodKind::Trait { body: true } | MethodKind::Inherent) => true,
             _ => {
-                self.tcx.sess.struct_span_err(attr.span, "attribute should be applied to a function definition")
-                    .span_label(*span, "not a function definition").emit();
+                self.tcx
+                    .sess
+                    .struct_span_err(
+                        attr.span,
+                        "attribute should be applied to a function definition",
+                    )
+                    .span_label(*span, "not a function definition")
+                    .emit();
             }
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/mbe.rs" "/checkout/compiler/rustc_passes/src/loops.rs" "/checkout/compiler/rustc_passes/src/naked_functions.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_rules.rs" "/checkout/compiler/rustc_passes/src/intrinsicck.rs" "/checkout/compiler/rustc_expand/src/mbe/transcribe.rs" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
