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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 675:
                         if let Some(((_, trait_did, name), rhs)) =
                             proj.as_ref().and_then(|(lhs, rhs)| Some((lhs.projection()?, rhs)))
                         {
-                            impl_trait_proj.entry(param_idx).or_default().push((
-                                trait_did,
-                                rhs,
-                            ));
+                            impl_trait_proj
+                            impl_trait_proj
+                                .entry(param_idx)
+                                .or_default()
+                                .push((trait_did, name, rhs));
 
                         return None;
                         return None;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/unindent_comments/tests.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/passes/unindent_comments.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/types.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
