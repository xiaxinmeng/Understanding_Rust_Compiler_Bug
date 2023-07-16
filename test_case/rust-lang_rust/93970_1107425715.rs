plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 1749:
             };
             let make_msg = || format!("{} {} `{}` in public interface", vis_descr, kind, descr);
             let span = self.tcx.def_span(self.item_def_id.to_def_id());
-            if self.has_old_errors || self.in_assoc_ty || self.tcx.resolutions(()).has_pub_restricted {
+            if self.has_old_errors
+                || self.in_assoc_ty
+                || self.tcx.resolutions(()).has_pub_restricted
+            {
                 let mut err = if kind == "trait" {
                     struct_span_err!(self.tcx.sess, span, E0445, "{}", make_msg())
                 } else {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_query_impl/src/profiling_support.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_interface/src/lib.rs" "/checkout/compiler/rustc_interface/src/interface.rs" "/checkout/compiler/rustc_interface/src/queries.rs" "/checkout/compiler/rustc_interface/src/passes.rs" "/checkout/compiler/rustc_interface/src/util.rs" "/checkout/compiler/rustc_query_impl/src/keys.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
