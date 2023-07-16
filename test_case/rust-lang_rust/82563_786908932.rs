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
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 839:
         let self_id = if item.is_fake() {
             None
         // Checking if the item is a field in a variant in an enum
-        } else if (matches!(self.cx.tcx.def_kind(item.def_id), DefKind::Field) && matches!(self.cx.tcx.def_kind(self.cx.tcx.parent(item.def_id).unwrap()), DefKind::Variant)) {
+        } else if (matches!(self.cx.tcx.def_kind(item.def_id), DefKind::Field)
+            && matches!(
+                self.cx.tcx.def_kind(self.cx.tcx.parent(item.def_id).unwrap()),
+                DefKind::Variant
+        {
+        {
             self.cx.tcx.parent(item.def_id).and_then(|item_id| self.cx.tcx.parent(item_id))
         } else if matches!(
             self.cx.tcx.def_kind(item.def_id),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/bootstrap/install.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/bootstrap/format.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
