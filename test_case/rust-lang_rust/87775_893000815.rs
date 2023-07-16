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
Diff in /checkout/compiler/rustc_resolve/src/late/diagnostics.rs at line 543:
 
 
                 // If the trait has a single item (which wasn't matched by Levenshtein), suggest it
-                if let Some(ident) = self.get_single_associated_item(&path, span, &source, is_expected)
+                if let Some(ident) =
+                    self.get_single_associated_item(&path, span, &source, is_expected)
                 {
                     err.note(format!("did you mean `{}`?", ident.name).as_str());
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_passes/src/feature_gate.rs" "/checkout/compiler/rustc_resolve/src/check_unused.rs" "/checkout/compiler/rustc_ast_passes/src/show_span.rs" "/checkout/compiler/rustc_resolve/src/macros.rs" "/checkout/compiler/rustc_ast_passes/src/ast_validation.rs" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs" "/checkout/compiler/rustc_resolve/src/imports.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
