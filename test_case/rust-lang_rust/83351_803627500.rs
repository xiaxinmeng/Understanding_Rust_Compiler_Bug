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
Diff in /checkout/compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs at line 79:
             mir::TerminatorKind::Drop { place: dropped_place, .. } => {
                 let dropped_ty = dropped_place.ty(self.body, self.tcx).ty;
                 if !NeedsDrop::in_any_value_of_ty(self.ccx, dropped_ty) {
-                    bug!("Drop elaboration left behind a Drop for a type that does not need dropping");
+                    bug!(
+                        "Drop elaboration left behind a Drop for a type that does not need dropping"
+                    );
 
 
                 if dropped_place.is_indirect() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/qualifs.rs" "/checkout/compiler/rustc_mir/src/transform/function_item_references.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/validation.rs" "/checkout/compiler/rustc_mir/src/transform/simplify.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
