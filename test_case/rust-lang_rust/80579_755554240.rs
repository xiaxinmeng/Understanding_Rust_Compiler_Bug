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
Diff in /checkout/compiler/rustc_mir/src/transform/promote_consts.rs at line 528:
                                 match ty.kind() {
                                     ty::Array(_, len) => {
                                         // It's an array; determine its length.
-                                        if let Some(len) = len.try_eval_usize(self.tcx, self.param_env) {
+                                        if let Some(len) =
+                                            len.try_eval_usize(self.tcx, self.param_env)
+                                        {
                                             // If the index is in-bounds, go ahead.
                                             if idx < u128::from(len) {
                                                 promotable = true;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/promote_consts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc_mir/src/transform/promote_consts.rs at line 535:
                                         }
                                     }
-                                    _ => {},
+                                    _ => {}
