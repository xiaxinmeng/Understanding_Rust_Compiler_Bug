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
Diff in /checkout/compiler/rustc_resolve/src/late/diagnostics.rs at line 2087:
                     }
                     fn suggestion(&self, sugg: String) -> Option<(Span, String)> {
                         Some(
-                            match (self.is_unnamed(), self.is_underscore(), self.is_named(), sugg.starts_with("&")) {
+                            match (
+                                self.is_unnamed(),
+                                self.is_underscore(),
+                                self.is_named(),
+                                sugg.starts_with("&"),
+                            ) {
                                 (true, _, _, false) => (self.span_unnamed_borrow(), sugg),
                                 (true, _, _, true) => {
                                     (self.span_unnamed_borrow(), sugg[1..].to_string())
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs" "/checkout/compiler/rustc_query_impl/src/values.rs" "/checkout/compiler/rustc_query_impl/src/lib.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_resolve/src/build_reduced_graph.rs" "/checkout/compiler/rustc_query_impl/src/plumbing.rs" "/checkout/compiler/rustc_resolve/src/lib.rs" "/checkout/compiler/rustc_query_impl/src/keys.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
