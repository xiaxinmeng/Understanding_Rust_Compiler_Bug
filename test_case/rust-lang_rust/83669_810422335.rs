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
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 2391:
                             // Add check case for similarly named item in alternative namespace
                             let mut suggestion = None;
 
-                            if ribs.is_some() &&
-                            self.resolve_ident_in_lexical_scope(
-                                    ValueNS,
-                                    parent_scope,
-                                    None,
-                                    path_span,
-                                    path_span,
-                                    &ribs.unwrap()[ValueNS],
-                                .is_some()
-                                .is_some()
+                            if ribs.is_some()
+                                && self
+                                    .resolve_ident_in_lexical_scope(
+                                        ValueNS,
+                                        parent_scope,
+                                        None,
+                                        path_span,
+                                        path_span,
+                                        &ribs.unwrap()[ValueNS],
+                                    .is_some()
                             {
                                 suggestion = Some((
                                 suggestion = Some((
                                     vec![(path_span, format!("{}", ident))],
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_feature/src/lib.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_feature/src/active.rs" "/checkout/compiler/rustc_resolve/src/diagnostics.rs" "/checkout/compiler/rustc_feature/src/removed.rs" "/checkout/compiler/rustc_resolve/src/build_reduced_graph.rs" "/checkout/compiler/rustc_resolve/src/imports.rs" "/checkout/compiler/rustc_resolve/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
