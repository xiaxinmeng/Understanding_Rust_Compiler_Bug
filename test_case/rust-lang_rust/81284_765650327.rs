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
Diff in /checkout/src/librustdoc/clean/utils.rs at line 487:
     ty: Ty<'tcx>,
     param_env_def_id: DefId,
 ) -> impl Iterator<Item = Item> {
-    let auto_impls = cx.sess().prof.generic_activity("get_auto_trait_impls").run(|| {
-        AutoTraitFinder::new(cx).get_auto_trait_impls(ty, param_env_def_id)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-    });
-    let blanket_impls = cx.sess().prof.generic_activity("get_blanket_impls").run(|| {
-        BlanketImplFinder::new(cx).get_blanket_impls(ty, param_env_def_id)
-    });
+    let auto_impls = cx
+        .sess()
+        .prof
+        .generic_activity("get_auto_trait_impls")
+        .run(|| AutoTraitFinder::new(cx).get_auto_trait_impls(ty, param_env_def_id));
+    let blanket_impls = cx
+        .sess()
+        .prof
+        .generic_activity("get_blanket_impls")
+        .run(|| BlanketImplFinder::new(cx).get_blanket_impls(ty, param_env_def_id));
     auto_impls.into_iter().chain(blanket_impls)
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:17
