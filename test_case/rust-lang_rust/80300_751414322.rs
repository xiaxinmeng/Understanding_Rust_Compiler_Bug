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
Diff in /checkout/src/librustdoc/visit_ast.rs at line 378:
                     .meta_item()
                     .map_or(false, |meta| CRATE_LEVEL_ATTRS.iter().any(|attr| meta.path == *attr))
-                let mut err = self.cx
-                let mut err = self.cx
-                    .sess()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/visit_ast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-                    .struct_span_warn(attr.meta_item().unwrap().span, "this attribute can only be applied at the crate level");
+                let mut err = self.cx.sess().struct_span_warn(
+                    attr.meta_item().unwrap().span,
+                    "this attribute can only be applied at the crate level",
+                );
                 err.note("read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information");
                 if self.cx.tcx.parent(item.def_id()).map_or(false, DefId::is_top_level_module) {
                     if let Ok(mut src) = self.cx.sess().source_map().span_to_snippet(attr.span()) {
Build completed unsuccessfully in 0:00:20
