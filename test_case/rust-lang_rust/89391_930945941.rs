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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/pat.rs at line 1404:
         )
         .span_label(span, format!("multiple uses of `{}` in pattern", ident))
         .span_label(other_field, format!("first use of `{}`", ident))
-        .span_suggestion(span, "consider removing one usage of", ident.to_string(), rustc_errors::Applicability::MachineApplicable)
+        .span_suggestion(
+            span,
+            "consider removing one usage of",
+            ident.to_string(),
+        )
+        )
         .emit();
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_passes/src/region.rs" "/checkout/compiler/rustc_typeck/src/collect/type_of.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/collect/item_bounds.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/dropck.rs" "/checkout/compiler/rustc_typeck/src/check/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
