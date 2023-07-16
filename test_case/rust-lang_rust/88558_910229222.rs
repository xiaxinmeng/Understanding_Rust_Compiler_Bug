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
Diff in /checkout/compiler/rustc_mir/src/transform/check_consts/check.rs at line 988:
 
                 // Check to see if the type of this place can ever have a drop impl. If not, this
                 // `Drop` terminator is frivolous.
-                let ty_needs_drop =
-                    dropped_place.ty(self.body, self.tcx).ty.needs_non_const_drop(self.tcx, self.param_env);
+                let ty_needs_drop = dropped_place
+                    .ty(self.body, self.tcx)
+                    .ty
+                    .needs_non_const_drop(self.tcx, self.param_env);
                 if !ty_needs_drop {
                     return;
                     return;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/qualifs.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/check.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_mir/src/transform/check_unsafety.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_mir/src/transform/remove_storage_markers.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
