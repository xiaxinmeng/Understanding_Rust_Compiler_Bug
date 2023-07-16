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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 1775:
                 } else {
                     struct_span_err!(self.tcx.sess, span, E0446, "{}", make_msg())
-                let vis_span =
-                let vis_span =
-                    self.tcx.source_map(()).guess_head_span(self.tcx.def_span(def_id));
+                let vis_span = self.tcx.source_map(()).guess_head_span(self.tcx.def_span(def_id));
                 err.span_label(span, format!("can't leak {} {}", vis_descr, kind));
                 err.span_label(vis_span, format!("`{}` declared as {}", descr, vis_descr));
                 err.emit();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/tests.rs" "/checkout/compiler/rustc_interface/src/util.rs" "/checkout/compiler/rustc_expand/src/mut_visit/tests.rs" "/checkout/compiler/rustc_interface/src/tests.rs" "/checkout/compiler/rustc_interface/src/callbacks.rs" "/checkout/compiler/rustc_expand/src/tokenstream/tests.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_interface/src/passes.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
